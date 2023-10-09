pub mod commands;
pub mod errors;
pub mod helpers;
pub mod states;
pub mod events;

use serde::{Deserialize, Serialize};

use crate::winapi::*;

use self::{errors::ProcessError, helpers::create_temp_process_from_entry};

pub type ProcessHandle = HANDLE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemporaryProcess {
    pub pid: u32,
    pub name: String,
}

pub struct Process {
    pub pid: u32,
    pub handle: ProcessHandle,
}

impl Process {
    pub unsafe fn find_all() -> anyhow::Result<Vec<TemporaryProcess>> {
        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if Process32First(snapshot, &mut entry) != TRUE {
            return Err(ProcessError::FailedSnapshot {
                error_code: GetLastError(),
            }
            .into());
        }

        let mut processes: Vec<TemporaryProcess> = vec![];
        // Pushes the first process from Process32First
        processes.push(create_temp_process_from_entry(&entry));

        // Loops through each process and adds them to a vector
        while Process32Next(snapshot, &mut entry) == TRUE {
            processes.push(create_temp_process_from_entry(&entry));
        }

        Ok(processes)
    }
}
