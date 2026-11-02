#!/bin/sh
# Regenerate every app icon from the brand source (Shei Designs PRODUCTS.md §1.3).
# monocycle-icon.svg is the canonical tile + mark (also in the website repo,
# apps/sheidesigns.com/design-reference/products/). macOS gets the same tile
# inset to 82% of the box, since Apple's icons sit smaller in their slot.
# Needs rsvg-convert and the Tauri CLI (desktop/node_modules). Run from repo root.
set -e
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
rsvg-convert -w 1024 -h 1024 "$here/monocycle-icon.svg" -o "$tmp/icon.png"
rsvg-convert -w 1024 -h 1024 "$here/monocycle-icon-macos.svg" -o "$tmp/icon-macos.png"
cd desktop
pnpm tauri icon "$tmp/icon.png" -o src-tauri/icons
pnpm tauri icon "$tmp/icon-macos.png" -o "$tmp/mac"
cp "$tmp/mac/icon.icns" src-tauri/icons/icon.icns
# Desktop-only app: drop the mobile sets the CLI also writes.
rm -rf src-tauri/icons/android src-tauri/icons/ios
rm -rf "$tmp"
