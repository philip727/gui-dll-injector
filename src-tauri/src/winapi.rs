pub use winapi::shared::minwindef::{FALSE, TRUE};

pub use winapi::shared::ntdef::{HANDLE, NULL};

pub use winapi::um::errhandlingapi::GetLastError;

pub use winapi::um::processthreadsapi::OpenProcess;

pub use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};

pub use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS};

pub use winapi::um::memoryapi::VirtualAllocEx;
