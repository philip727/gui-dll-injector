use std::ffi::c_void; use std::path::Path;
use std::ptr;

use crate::winapi::*;

use crate::{
    c::helpers::to_wide_string,
    inject::{errors::InjectionError, Injectable},
};

use super::Process;

impl Injectable for Process {
    unsafe fn inject(&self, dll: &str) -> anyhow::Result<()> {
        let dll_path = Path::new(dll).to_str().ok_or(InjectionError::InvalidPath {
            path: dll.to_string(),
        })?;
        let wstr_dll_path = to_wide_string(&dll_path);
        let dll_path_size = wstr_dll_path.len() * 2 + 1;

        // Allocate the space required for the dll with the right permissions
        let mem_buffer = VirtualAllocEx(
            self.handle,
            NULL,
            dll_path_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );

        // Failed to allocate mem
        if mem_buffer.is_null() {
            CloseHandle(self.handle);
            return Err(InjectionError::FailedVirtualAlloc {
                error_code: GetLastError(),
            }
            .into());
        }

        // Writes the data to the allocated memory
        if WriteProcessMemory(
            self.handle,
            mem_buffer,
            wstr_dll_path.as_ptr() as *const c_void,
            dll_path_size,
            ptr::null_mut(),
        ) == FALSE
        {
            CloseHandle(self.handle);
            return Err(InjectionError::FailedMemoryWrite {
                error_code: GetLastError(),
                pid: self.pid,
            }
            .into());
        }

        // Null terminated kernel 32 string
        let sz_kernel_32 = "Kernel32\0".as_ptr();
        // Retrieves the kernel32 module handle
        let kernel_32_module = GetModuleHandleA(sz_kernel_32 as *const i8);

        // Cleans up if its null
        if kernel_32_module.is_null() {
            CloseHandle(self.handle);
            return Err(InjectionError::FailedToLoadModule {
                error_code: GetLastError(),
                name: "Kernel32".to_string(),
            }
            .into());
        }

        let start_routine: LPTHREAD_START_ROUTINE = {
            let sz_load_library = "LoadLibraryW\0".as_ptr();
            let proc_addr = GetProcAddress(kernel_32_module, sz_load_library as *const i8);

            std::mem::transmute(proc_addr)
        };

        let tid: LPDWORD = std::ptr::null_mut();
        // Create a thread in the virtual space of target process
        let remote_thread = CreateRemoteThread(
            self.handle,
            ptr::null_mut(),
            0,
            start_routine,
            mem_buffer,
            0,
            tid,
        );

        if remote_thread.is_null() {
            CloseHandle(self.handle);
            return Err(InjectionError::FailedToCreateRemoteThread {
                error_code: GetLastError(),
            }
            .into());
        }

        // Cleanup handles
        CloseHandle(remote_thread);
        CloseHandle(self.handle);

        Ok(())
    }
}
