# MonoCycle — desktop shell

This directory is the [Tauri 2](https://tauri.app/) wrapper that turns MonoCycle
into a native desktop app. For what MonoCycle *is*, how to install it, and how
to use it, see the [root README](../README.md).

## Layout

```
desktop/
├── src/
│   ├── index.html          the entire application — markup, CSS and DSP
│   └── fonts/              bundled JetBrains Mono + VT323 (SIL OFL 1.1)
└── src-tauri/
    ├── src/lib.rs          Tauri setup + the two commands the frontend calls
    ├── src/main.rs         binary entry point
    ├── tauri.conf.json     window, bundle metadata, app identifier
    ├── capabilities/       ACL permissions granted to the main window
    └── icons/              app icons for every platform
```

There is no bundler, no framework and no build step for the frontend. Tauri
serves `src/` as-is. `index.html` is the whole app: a `<style>` block, the
markup, and one `<script>` block holding the DSP, the audio graph, the canvas
drawing and the UI wiring. Edit it and reload.

## Commands

```sh
pnpm install          # once
pnpm tauri dev        # dev build, hot-reloads frontend edits
pnpm tauri build      # release bundle -> src-tauri/target/release/bundle/
```

`pnpm tauri dev` compiles ~500 crates on the first run and takes a few minutes.
After that it launches in a second or two, and edits to `index.html` appear on
reload without recompiling Rust.

Release builds use `lto = true` and `codegen-units = 1` (see
`src-tauri/Cargo.toml`), so they are considerably slower to compile than dev
builds. That is deliberate — release binaries are what users download.

### Building an AppImage on Arch

`linuxdeploy` and its GStreamer plugin both assume a Debian filesystem layout,
so on Arch a plain `pnpm tauri build` fails outright and then, once it stops
failing, produces an AppImage that warns on every launch. Build it like this:

```sh
# Stage just the two GStreamer helper binaries linuxdeploy expects to find.
HELPERS=$(mktemp -d)
cp /usr/lib/gstreamer-1.0/gst-plugin-scanner \
   /usr/lib/gstreamer-1.0/gst-ptp-helper "$HELPERS"/

NO_STRIP=1 \
APPIMAGE_EXTRACT_AND_RUN=1 \
GSTREAMER_HELPERS_DIR="$HELPERS" \
  pnpm tauri build

rm -rf "$HELPERS"
```

- `APPIMAGE_EXTRACT_AND_RUN=1` — `linuxdeploy` is itself an AppImage and needs
  FUSE to mount itself. This makes it extract and run instead. Without it the
  build dies with a bare `failed to run linuxdeploy`.
- `NO_STRIP=1` — `linuxdeploy` bundles an old `strip` that cannot parse the
  `.relr.dyn` section modern libraries carry, and treats the failure as fatal.
  Our own binary is already stripped by the release profile, so nothing is lost.
- `GSTREAMER_HELPERS_DIR` — the GStreamer plugin script falls back to
  `/usr/lib/gstreamer-1.0` when hunting for *plugins*, but its **helpers** path
  is hardcoded to `/usr/lib/<arch>-linux-gnu/gstreamer1.0/gstreamer-1.0` with no
  fallback. On Arch that misses, `gst-plugin-scanner` never gets bundled, and
  the AppRun hook still exports a path to it that does not exist. GStreamer then
  spams `External plugin loader failed` on every launch and silently falls back
  to loading plugins in-process — losing the crash isolation the scanner exists
  to provide. Pointing at `/usr/lib/gstreamer-1.0` directly also works but
  re-copies all 272 plugins as "helpers"; staging the two binaries keeps the
  AppImage at 161 MB instead of 169 MB.

A correct AppImage launches with **no output at all**. If you see GStreamer
warnings, one of the three is missing.

`.deb` and `.rpm` need none of this — they use the system GStreamer. **The
release workflow does not set `GSTREAMER_HELPERS_DIR`, and must not**: the CI
runners are Ubuntu, where the script's default helpers path is the correct one.
Hardcoding the Arch path there would break it.

### The AppImage must bundle GStreamer

`tauri.conf.json` sets `bundle.linux.appimage.bundleMediaFramework: true`.
**Do not turn this off.** WebKitGTK implements the Web Audio API on top of
GStreamer, and Tauri's AppImage otherwise ships GStreamer's core libraries with
*no plugins* — the bundled core then shadows the host's, no audio sink can be
created, and every attempt to play fails with "Failed to start the audio
device". The app looks completely normal and is completely silent.

It is off by default and costs roughly 60 MB (the AppImage goes from ~99 MB to
~161 MB), which is why it is easy to miss and why it is worth the bytes here.

To check it survived a change to the bundle config:

```sh
# should print a few hundred, not 0
find src-tauri/target/release/bundle/appimage/*.AppDir \
     -path '*gstreamer-1.0*' -name '*.so' | wc -l
```

The `.deb` and `.rpm` are unaffected — they link against the system GStreamer,
pulled in as a dependency of `libwebkit2gtk-4.1-0`.

## The Rust side

The frontend is self-sufficient for everything except two things the browser
sandbox cannot do, so `lib.rs` is deliberately tiny:

- **`save_wav(path, bytes)`** — writes the exported WAV. The frontend opens a
  native save dialog via `tauri-plugin-dialog` to get a user-blessed path, then
  hands the bytes over. Doing it this way avoids pulling in `tauri-plugin-fs`
  and its whole scope/permission system for one file write. The `<a download>`
  trick that works in a normal browser does not fire in WebKitGTK.
- **`app_version()`** — returns `CARGO_PKG_VERSION` for the About panel, so the
  displayed version can never drift from the manifests.

Both are plain `#[tauri::command]`s registered through `generate_handler!`.
App-defined commands are not gated by the capability ACL — that only governs
core and plugin commands — so `capabilities/default.json` only needs the
`core`, `opener` and `dialog` default sets.

`lib.rs` also sets `WEBKIT_DISABLE_DMABUF_RENDERER=1` on Linux before WebKit
initializes. WebKitGTK's DMA-BUF renderer crashes on some Wayland setups
(notably Arch with webkit2gtk-4.1 ≥ 2.50). An existing value in the environment
is respected, so users can override it.

## Releasing

Version lives in **three** manifests and they must agree:

- `package.json` → `version`
- `src-tauri/Cargo.toml` → `version`
- `src-tauri/tauri.conf.json` → `version`

The About panel reads its version from the compiled binary, so it follows
`Cargo.toml` automatically — do not hardcode a version in `index.html`.

To cut a release: bump those three, update `CHANGELOG.md`, commit, then push a
tag matching `v*`. `.github/workflows/release.yml` builds Linux x86_64, Linux
aarch64 (Raspberry Pi), Windows and macOS, and opens a **draft** release with
the installers attached. Review it, then publish.

You can dry-run the whole matrix without tagging by triggering the workflow
manually from the Actions tab.

## Security notes

- **No network access.** The app fetches nothing at runtime. Fonts used to come
  from Google Fonts; they are now bundled in `src/fonts/` so the app renders
  correctly offline and makes no outbound requests.
- **`csp` is `null`** in `tauri.conf.json`. The whole app is one inline
  `<script>`, and Tauri injects a nonce into `script-src` when a CSP is set —
  which makes browsers ignore `'unsafe-inline'` and block the app's own script.
  Enabling a CSP therefore requires extracting the script to an external file
  first. Worth doing, but it is a structural change, not a config tweak. The
  practical exposure is low: no remote content is ever loaded and no untrusted
  input is ever parsed as HTML.
- **`save_wav` writes wherever it is told.** That is safe because the only
  caller path runs through the native save dialog, which means the user picked
  the destination. Do not call it with a path the user has not chosen.

## Recommended IDE setup

[VS Code](https://code.visualstudio.com/) +
[Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
