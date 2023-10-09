use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum InjectionError {
    #[error("Couldn't find DLL at path. \"{path}\"")]
    InvalidPath { path: String },
    #[error("Couldn't create mem buffer, error: {error_code}")]
    FailedVirtualAlloc {
        error_code: u32,
    },
    #[error("Failed to write data to memory in process({pid}), error: {error_code}")]
    FailedMemoryWrite {
        error_code: u32,
        pid: u32,
    },
    #[error("Failed to load module handle for \"{name}\", error: {error_code}")]
    FailedToLoadModule {
        error_code: u32,
        name: String
    },
    #[error("Failed to create remote thread, error: {error_code}")]
    FailedToCreateRemoteThread {
        error_code: u32,
    },
}
