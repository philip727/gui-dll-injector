// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use process::states::SelectedProcess;

use crate::process::commands::{get_all_processes, open_process_list, get_selected_process, set_selected_process};
use crate::inject::commands::inject_dll;

pub mod inject;
pub mod process;
pub mod winapi;
pub mod c;

fn main() {
    tauri::Builder::default()
        .manage(SelectedProcess(Arc::new(Mutex::new(None))))
        .invoke_handler(tauri::generate_handler![
            get_all_processes,
            open_process_list,
            set_selected_process,
            inject_dll,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
