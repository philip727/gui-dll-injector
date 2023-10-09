use crate::process::Process;

use super::Injectable;

#[tauri::command]
pub fn inject_dll(dll: String, pid: u32) -> Result<(), String> {
    unsafe {
        let process = Process::new(pid).map_err(|e| e.to_string())?;
        let inject = process.inject(&dll).map_err(|e| e.to_string())?;
    }

    Ok(())
}
