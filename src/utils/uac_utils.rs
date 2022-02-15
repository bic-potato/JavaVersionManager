use std::ffi::CString;
use std::mem::size_of_val;
use std::ptr;
use std::ptr::null;
use winapi::shared::minwindef::{DWORD, PDWORD};
use winapi::shared::ntdef::NULL;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winnt::{HANDLE, LPCSTR, LUID, LUID_AND_ATTRIBUTES, SE_CREATE_SYMBOLIC_LINK_NAME, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES};
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winbase::LookupPrivilegeValueA;


#[cfg(windows)]
pub unsafe fn get_privilage() -> bool {
    let mut hToken: HANDLE = NULL;
    let mut token = &mut hToken;
    let mut process_handle = OpenProcessToken(
        GetCurrentProcess(),
        TOKEN_ADJUST_PRIVILEGES,
        token,
    );
    println!("{}", process_handle);
    let mut luid: LUID = Default::default();
    let mut luid_pointer = &mut luid;
    let privilage = CString::new("SeCreateSymbolicLinkPrivilege").unwrap();
    let name_ptr: *const i8 = privilage.as_ptr() as *const i8;
    let lookup = LookupPrivilegeValueA(
        0 as LPCSTR,
        name_ptr,
        luid_pointer,
    );
    println!("{}", lookup);
    println!("0x{:08X}", GetLastError());
    let mut structs = LUID_AND_ATTRIBUTES {
        Luid: luid,
        Attributes: SE_PRIVILEGE_ENABLED,
    };

    let mut tokenstate = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [structs],
    };
    let mut token_pre = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [structs],
    };
    let return_len = size_of_val(&token);
    let mut return_length = DWORD::try_from(return_len).unwrap();
    let secure = AdjustTokenPrivileges(
        hToken,
        0,
        &mut tokenstate,
        DWORD::try_from(size_of_val(&token_pre)).unwrap(),
        &mut token_pre,
        &mut return_length
    );
    if secure != 0 {
        return true
    } else {
        println!("0x{:08X}", GetLastError());
        return false}
}

#[cfg(test)]
mod uac_test{
    use crate::utils::uac_utils;
    #[test]
    fn uac_tests () {
        unsafe {
            let boolean = uac_utils::get_privilage();

            println!("{}", boolean);
        }
    }
}