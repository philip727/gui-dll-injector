use std::sync::Arc;

use tauri::{Manager, State};

use super::{events::ProcessSelectedEvent, states::SelectedProcess, Process, TemporaryProcess};

#[tauri::command]
pub fn get_all_processes() -> Result<Vec<TemporaryProcess>, String> {
    unsafe { Process::find_all().map_err(|e| e.to_string()) }
}

#[tauri::command]
pub async fn open_process_list(handle: tauri::AppHandle) {
    let _ = tauri::WindowBuilder::new(
        &handle,
        "proclist",
        tauri::WindowUrl::App("proclist".into()),
    )
    .title("Process List")
    .decorations(false)
    .resizable(false)
    .inner_size(300., 400.)
    .build()
    .unwrap();
}

#[tauri::command]
pub fn set_selected_process(
    handle: tauri::AppHandle,
    selected_process: State<SelectedProcess>,
    name: String,
    pid: u32,
) {
    let mut state = selected_process.0.lock().unwrap();

    handle
        .emit_all("selected_pid", ProcessSelectedEvent { name: &name, pid })
        .unwrap();

    *state = Some(TemporaryProcess { name, pid })
}

#[tauri::command]
pub fn get_selected_process(selected_process: State<SelectedProcess>) -> Option<TemporaryProcess> {
    let state = selected_process.0.lock().unwrap();

    state.clone().as_ref().cloned()
}
