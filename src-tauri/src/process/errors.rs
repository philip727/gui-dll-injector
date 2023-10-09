use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ProcessError {
    #[error("Couldn't get a snapshot of the processes, error: {error_code}.")]
    FailedSnapshot {
        error_code: u32,
    }
}
