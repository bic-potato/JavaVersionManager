use std::error::Error;
use std::fs::File;
use crate::utils::{uac_utils, ziputil};
use console::Style;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::{fs, path, result};
use std::path::Path;
use std::path::PathBuf;
use toml;
use crate::utils::release_utils::ReleaseParser;


#[derive(Serialize, Deserialize)]
pub struct Java {
    full_version: String,
    jvm_variant: String,
    image_type: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct JavaNew {
    implementor: String,
    full_version: String,
    jvm_variant: String,
    image_type: String,
    path: String,
}

impl JavaNew {
    pub fn new(implementor: &str,
               full_version: &str,
               jvm_variant: &str,
               image_type: &str,
               path: &str) -> JavaNew
    {
        let implementor = implementor.to_string();
        let full_version = full_version.to_string();
        let jvm_variant = jvm_variant.to_string();
        let image_type = image_type.to_string();
        let path = path.to_string();
        return JavaNew { implementor, full_version, jvm_variant, image_type, path };
    }
    pub fn get_implementor(&mut self) -> String {
        let out = self.implementor.to_owned();
        return out;
    }
    pub fn get_full_version(&mut self) -> String {
        let out = self.full_version.to_owned();
        return out;
    }
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
pub struct Store {
    Java_Version: Option<Vec<Java>>,
}

#[derive(Serialize, Deserialize)]
pub struct StoreNew {
    java_version: Option<Vec<JavaNew>>,
}

impl StoreNew {
    pub fn new() -> StoreNew {
        let mut vec: Vec<JavaNew> = Vec::new();
        return StoreNew { java_version: Some(vec) };
    }
    pub fn add(&mut self, obj: JavaNew) {
        let mut list = self.java_version.as_mut().unwrap();
        list.push(obj);
    }
    pub fn get_java_versions(&mut self) -> &mut Vec<JavaNew> {
        let mut list = self.java_version.as_mut().unwrap();
        return list;
    }

    pub fn get_full_version_list(&mut self) -> Vec<String> {
        let mut list = self.java_version.as_mut().unwrap();
        let mut version_list: Vec<String> = Vec::new();
        for element in list {
            version_list.push(element.full_version.to_string());
        }
        version_list
    }


    pub fn contains(&mut self, version: &str, implementor: &str) -> bool {
        let list = self.java_version.as_ref().unwrap();

        for element in list {
            if element.full_version == String::from(version) && element.implementor == String::from(implementor) {
                return true;
            }
        }
        return false;
    }
}

pub fn old_to_new(store: Store) -> StoreNew {
    let mut new = StoreNew::new();
    if let Some(list) = store.Java_Version {
        for element in list {
            let java_new = JavaNew::new("Eclipse Adoptium", &element.full_version, &element.jvm_variant, &element.image_type, &element.path);
            new.add(java_new);
        }
    } else {
        panic!("Error on converting record file!");
    }
    return new;
}


pub fn read_version() -> StoreNew {
    let mut current_location = std::env::current_exe().unwrap();
    current_location.pop();
    current_location.push("versions.toml");
    let contents = std::fs::read_to_string(&current_location).expect("Unable to load Version Files");
    let mut result: Store = toml::from_str(&contents).unwrap();
    if let Some(store) = result.Java_Version {
        let javaverisions = old_to_new( toml::from_str(&contents).unwrap());
        let content_string = toml::to_string(&javaverisions).unwrap();
        fs::write(&current_location, content_string).unwrap();
        return javaverisions;
    } else {
        let result: StoreNew = toml::from_str(&contents).unwrap();
        return result;
    }
}


pub fn version_record(java_config: JavaNew) {
    let mut version_file = std::env::current_exe().unwrap();
    version_file.pop();
    version_file.push("versions.toml");
    let mut conf = read_version();
    conf.add(java_config);
    let config = toml::to_string(&conf).unwrap();
    fs::write(&version_file, config.as_bytes()).expect("Err");
}

pub fn enable_version(implementor: &str, version: &str) {
    let store: StoreNew = read_version();
    if let Some(lists) = store.java_version {
        for element in lists {
            if version == element.full_version && implementor == element.implementor {
                let mut path = PathBuf::new();
                path.push(&element.path);
                //path.push("bin/");
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("OpenJDK/");
                // unsafe {
                //     uac_utils::get_privilage();
                // }
                let _ = std::fs::remove_dir_all(&current_location);

                let result = std::os::windows::fs::symlink_dir(&path, &current_location);
                match result {
                    Ok(_) => {
                        let green = Style::new().green();
                        println!("{}, JDK VERSION:{}", green.apply_to("Enable SUCCESS"), version)
                    }
                    Err(e) => {
                        let red = Style::new().red();
                        println!("{}, {}", red.apply_to("Enable FAILED"), e.to_string());
                    }
                }
            }
        }
    }
}


pub fn read_local(path: &str) {
    let mut record = read_version();
    let mut release_parser = ReleaseParser::new(path);
    let java = release_parser.parse();

    if !record.contains(&java.full_version, &java.implementor) {
        version_record(java);
        let green = Style::new().green();
        println!("{}", green.apply_to("jdk install finish!"))
    } else {
        let red = Style::new().red();
        println!("{}JDK already exist!", red.apply_to("Error"));
    }
}

