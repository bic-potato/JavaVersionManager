/*
 * @Author: ZuoXichen
 * @Date: 2022-01-30 11:56:36
 * @LastEditTime: 2023-01-01 15:09:17
 * @LastEditors: ZuoXichen
 * @Description:
 */
use crate::java_ver;
use download_rs::async_download::Download;
use reqwest;
use console::Style;
use serde::{ Deserialize, Serialize };
use crate::utils::ziputil;

#[derive(Serialize, Deserialize)]
struct JsonPackage {
    checksum: String,
    checksum_link: Option<String>,
    download_count: i32,
    link: String,
    metadata_link: Option<String>,
    name: String,
    signature_link: Option<String>,
    size: i32,
}

#[derive(Serialize, Deserialize)]
struct JsonVersion {
    build: i32,
    major: i32,
    minor: i32,
    openjdk_version: String,
    security: i32,
    semver: String,
}

#[derive(Serialize, Deserialize)]
struct JsonBinary {
    architecture: String,
    download_count: i32,
    heap_size: String,
    image_type: String,
    jvm_impl: String,
    os: String,
    installer: Option<JsonPackage>,
    package: JsonPackage,
    project: String,
    scm_ref: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
struct JsonInfo {
    binary: JsonBinary,
    release_link: String,
    release_name: String,
    vendor: String,
    version: JsonVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRelease {
    pub available_lts_releases: Vec<i32>,
    pub available_releases: Vec<i32>,
    pub most_recent_feature_release: i32,
    pub most_recent_feature_version: i32,
    pub most_recent_lts: i32,
    pub tip_version: i32,
}

pub async fn list_remote() -> JsonRelease {
    let url = String::from(
        "https://api.adoptium.net/v3/info/available_releases?image_type=jdk&os=windows&page=0&release_type=ga&sort_order=ASC"
    );
    // println!("{}", url);
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let result: JsonRelease = serde_json::from_str(&request).unwrap();
    return result;
}

pub async fn get_remote(version: i32) {
    let url =
        format!("https://api.adoptium.net/v3/assets/latest/{}/hotspot?vendor=eclipse", version);
    let red = Style::new().red();
    //println!("{}", url);
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let result: Vec<JsonInfo> = serde_json::from_str(&request).unwrap();
    // let mut arch = "";
    // if std::env::consts::ARCH == "x86_64" {
    //     arch = "x64";
    // } else if std::env::consts::ARCH == "x86" {
    //     arch = "x32";
    // }
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "x86" => "x32",
        &_ => "unknown",
    };
    if result.len() == 0 {
        eprintln!("{} JDK not found!", red.apply_to("Error"));
        return;
    }

    for assets in result {
        // println!("{}",result[i]["binary"].pretty(4));
        if
            assets.binary.image_type == "jdk" &&
            assets.binary.os == "windows" &&
            arch == assets.binary.architecture
        {
            let file_name = assets.binary.package.name;
            let version_name = assets.release_name;
            let mut version_store = java_ver::read_version();

            if !version_store.contains(&version_name, "Eclipse Adoptium") {
                // println!("{}", file_name);
                let download_url_tuna = format!(
                    "https://mirrors.tuna.tsinghua.edu.cn/Adoptium/{}/jdk/{}/windows/{}",
                    version,
                    arch,
                    file_name
                );

                let download_url_nju = format!(
                    "https://mirrors.tuna.tsinghua.edu.cn/Adoptium/{}/jdk/{}/windows/{}",
                    version,
                    arch,
                    file_name
                );
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("temp/");
                let save_location = current_location.to_str().unwrap();
                // println!("{}", save_location);
                let download_url_vec = vec![
                    download_url_tuna,
                    download_url_nju,
                    assets.binary.package.link.to_owned()
                ];

                for download_url in download_url_vec {
                    match download_resources(&download_url, save_location).await {
                        Ok(_) => {
                            // println!("OK, {}",download_url);
                            record_java_data_to_file(&version_name);
                            unzip_and_transfer_java_files(&file_name);
                            break;
                        }
                        Err(e) => {
                            if download_url == assets.binary.package.link {
                                eprintln!(
                                    "{} Download JDK Failed, {}",
                                    red.apply_to("Error"),
                                    e.to_string()
                                );
                            }
                        }
                    }
                }
            } else {
                eprintln!("{}JDK already exist!", red.apply_to("Error"));
            }
        }
    }
}

fn record_java_data_to_file(version_name: &str) {
    let mut java_location = std::env::current_exe().unwrap();
    java_location.pop();
    java_location.push("java/");
    java_location.push(&version_name);
    let java_location_str = java_location.to_str().unwrap();
    let java_location_str = java_location_str.replace("\\", "/");
    let java: java_ver::JavaNew = java_ver::JavaNew::new(
        "Eclipse Adoptium",
        &version_name,
        "Hotspot",
        "jdk",
        &java_location_str
    );
    java_ver::version_record(java);
}

fn unzip_and_transfer_java_files(file_name: &str) {
    let green = Style::new().green();
    let red = Style::new().red();
    let mut current_location = std::env::current_exe().unwrap();
    current_location.pop();
    current_location.push("temp/");
    let save_location = current_location.to_str().unwrap().to_owned();
    current_location.pop();
    current_location.push("java/");
    let ziped_file = save_location + &file_name;
    ziputil::extract(std::path::Path::new(&ziped_file), &current_location);
    match std::fs::remove_file(std::path::Path::new(&ziped_file)) {
        Ok(_) => println!("{}", green.apply_to("JDK install finish!")),
        Err(e) => println!("{} temp file delete failed, {}", red.apply_to("Error"), e.to_string()),
    }
}
async fn download_resources(
    url: &str,
    save_location: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let download = Download::new(url, Some(save_location), None);
    return download.download_async().await;
}