use crate::utils::ziputil;
use console::Style;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use toml;
// use windows_sys::Win32::Foundation;
// use windows_sys::Win32::Foundation::HANDLE;
// use windows_sys::Win32::Foundation::LUID;
// use windows_sys::Win32::Security;
// use windows_sys::Win32::Security::TOKEN_ADJUST_PRIVILEGES;
// use windows_sys::Win32::System::SystemServices::SE_CREATE_SYMBOLIC_LINK_NAME;
// use windows_sys::Win32::System::Threading;

#[derive(Serialize, Deserialize)]
pub struct Java {
    full_version: String,
    jvm_variant: String,
    image_type: String,
    path: String,
}

impl Java {
    pub fn new(
        full_version: String,
        jvm_variant: String,
        image_type: String,
        path: String,
    ) -> Java {
        Java {
            full_version,
            jvm_variant,
            image_type,
            path,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Store {
    Java_Version: Option<Vec<Java>>,
}

pub fn read_version() -> Vec<String> {
    let mut current_location = std::env::current_exe().unwrap();
    current_location.pop();
    current_location.push("versions.toml");
    let contents = std::fs::read_to_string(&current_location).expect("Unable to load Version Files");
    let store: Store = toml::from_str(&contents).unwrap();
    let mut versions: Vec<String> = Vec::new();
    if let Some(list) = store.Java_Version {
        for element in list {
            let version = element.full_version;
            versions.push(version)
        }
    }
    return versions;
}

pub fn version_record(file: &Path, java_config: Java) {
    let mut current_location = std::env::current_exe().unwrap();
    current_location.pop();
    current_location.push("java/");
    ziputil::extract(&file, &current_location);
    let mut version_file = std::env::current_exe().unwrap();
    version_file.pop();
    version_file.push("versions.toml");
    let mut file = std::fs::File::options()
        .append(true)
        .open(&version_file)
        .unwrap();
    let mut list: Vec<Java> = Vec::new();
    list.push(java_config);
    let conf: Store = Store {
        Java_Version: Some(list),
    };
    let config = toml::to_string(&conf).unwrap();
    file.write(config.as_bytes()).expect("Err");
}

pub fn enable_version(version: &str) {
    let mut current_location = std::env::current_exe().unwrap();
    current_location.pop();
    current_location.push("versions.toml");
    let contents = std::fs::read_to_string(&current_location).expect("Unable to load Version Files");
    let store: Store = toml::from_str(&contents).unwrap();
    if let Some(list) = store.Java_Version {
        for element in list {
            if version == element.full_version {
                let mut path = PathBuf::new();
                path.push(&element.path);
                //path.push("bin/");
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("OpenJDK/");
                // unsafe {
                //     let hToken: HANDLE;
                //     let mut token = &mut hToken;
                //     let mut retn = Threading::OpenProcessToken(
                //         Threading::GetCurrentProcess(),
                //         TOKEN_ADJUST_PRIVILEGES,
                //         token,
                //     );
                //     let mut luid: LUID;
                //     let mut luid_pointer = &mut luid;
                //     let _ = Security::LookupPrivilegeValueA(
                //         None,
                //         &SE_CREATE_SYMBOLIC_LINK_NAME,
                //         luid_pointer,
                //     );
                //     let mut structs = Security::LUID_AND_ATTRIBUTES {
                //         Luid: luid,
                //         Attributes: Security::SE_PRIVILEGE_ENABLED,
                //     };

                //     let token = Security::TOKEN_PRIVILEGES {
                //         PrivilegeCount: 1,
                //         Privileges: [structs],
                //     };
                //     let secure = Security::AdjustTokenPrivileges(
                //         Threading::GetCurrentProcess(),
                //         0,
                //         &mut token,
                //         0,
                //         None,
                //         None
                //     );
                // }
                let result = std::fs::remove_dir_all(&current_location);

                let result = std::os::windows::fs::symlink_dir(&path, &current_location);
                match result {
                    Ok(_) => {
                        let green = Style::new().green();
                        println!("{}, JDK VERSION:{}",green.apply_to("Enable SUCCESS"), version)
                    }
                    Err(e) =>{
                        let red = Style::new().red();
                         println!("{}, {}", red.apply_to("Enable FAILED") ,e.to_string());
                    }
                }
            }
        }
    }
}
