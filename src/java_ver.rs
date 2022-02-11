use crate::utils::ziputil;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use toml;

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
    let contents = std::fs::read_to_string("./versions.toml").expect("Unable to load");
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
    let mut file = std::fs::File::options()
        .append(true)
        .open("./versions.toml")
        .unwrap();
    let mut list: Vec<Java> = Vec::new();
    list.push(java_config);
    let conf: Store = Store {
        Java_Version: Some(list),
    };
    let config = toml::to_string(&conf).unwrap();
    file.write(config.as_bytes()).expect("Err");
}

pub fn enable_version(version: &str){
    let contents = std::fs::read_to_string("./versions.toml").expect("Unable to load");
    let store: Store = toml::from_str(&contents).unwrap();
    if let Some(list) = store.Java_Version {
        for element in list {
            if version == element.full_version
            {
                let mut path = PathBuf::new();
                path.push(&element.path);
                //path.push("bin/");
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("OpenJDK/");
                let result = std::os::windows::fs::symlink_dir(&path, &current_location);
                match result {
                    Ok(_) => println!("Enable SUCCESS, JDK VERSION:{}", version),
                    Err(e) => println!("Enable FAILED, {}", e.to_string())
                }
            }
        }
    }
}