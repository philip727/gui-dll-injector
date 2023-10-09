use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ProcessError {
    #[error("Couldn't get a handle to the process {pid}, error: {error_code}.")]
    NoHandle {
        pid: u32,
        error_code: u32,
    },
    #[error("Couldn't get a snapshot of the processes, error: {error_code}.")]
    FailedSnapshot {
        error_code: u32,
    }
}
