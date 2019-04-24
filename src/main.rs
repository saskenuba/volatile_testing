#![allow(unused_imports)]

extern crate libc;
extern crate winapi;

use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem::{size_of, size_of_val};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::thread;

use chiter::make_entrypoint;
use volatile::Volatile;
use winapi::shared::minwindef::{DWORD, HMODULE, MAX_PATH};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW};
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::SetWindowsHookExA;
use winapi::um::winuser::{MessageBoxW, MB_HELP};

fn process_print_name_and_id(pid: u32) {
    println!("{}", pid);

    let mut szProcessPath: Vec<u16> = vec![0; MAX_PATH];

    // Get Handle of the process
    let h_mod: HMODULE = null_mut();
    let cbNeeded: DWORD = 0;

    unsafe {
        let process_handle: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION, PROCESS_VM_READ as i32, pid);

        if EnumProcessModules(process_handle, h_mod, size_of_val(h_mod) as u32, *cbNeeded) {
            GetModuleBaseNameW(
                process_handle,
                h_mod,
                szProcessPath.as_mut_ptr(),
                size_of_val(&szProcessPath) as u32,
            )
        }
    };

    println!("{:?}", szProcessPath);
}

fn main() {
    let mut teste: Vec<DWORD> = vec![0; 1024];
    let teste_ptr = teste.as_mut_ptr();
    let teste_size = size_of_val(&teste);

    unsafe {
        EnumProcesses(teste_ptr, teste_size as u32, null_mut());
    };

    println!("{:#?}", &teste);

    for pid in teste {
        if pid > 0 {
            process_print_name_and_id(pid);
        }
    }
}
