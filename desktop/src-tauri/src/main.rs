// MonoCycle — Copyright (C) 2026  Andrew O'Shei
// Licensed under GPL v3. See lib.rs or the LICENSE file at the repo root for
// full notice. <https://www.gnu.org/licenses/gpl-3.0.html>

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    monocycle_lib::run()
}
