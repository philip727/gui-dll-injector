use crate::winapi::*;

use super::TemporaryProcess;

pub fn create_temp_process_from_entry(entry: &PROCESSENTRY32) -> TemporaryProcess {
    let process_name = entry
        .szExeFile
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect::<Vec<u8>>();

    TemporaryProcess {
        pid: entry.th32ProcessID,
        name: String::from_utf8(process_name).unwrap(),
    }
}
