/*
 * @Author: ZuoXichen
 * @Date: 2022-01-30 11:56:36
 * @LastEditTime: 2022-05-12 23:49:20
 * @LastEditors: ZuoXichen
 * @Description: 
 */
use crate::java_ver;
use download_rs::async_download::Download;
use json;
use reqwest;
use console::Style;
use crate::utils::ziputil;

pub async fn list_remote() -> json::JsonValue {
    let url = String::from("https://api.adoptium.net/v3/info/available_releases?image_type=jdk&os=windows&page=0&release_type=ga&sort_order=ASC");
    println!("{}", url);
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let result = json::parse(&request).unwrap();
    return result;
}

pub async fn get_remote(version: &str) {
    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{}/hotspot?vendor=eclipse",
        version
    );
    let green = Style::new().green();
    let red = Style::new().red();
    //println!("{}", url);
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let result = json::parse(&request).unwrap();
    let mut i = 0;
    let mut file_name: &str;
    let mut arch = "";
    if std::env::consts::ARCH == "x86_64" {
        arch = "x64";
    } else if std::env::consts::ARCH == "x86" {
        arch = "x32";
    }
    while i < result.len() {
        // println!("{}",result[i]["binary"].pretty(4));
        if result[i]["binary"]["image_type"] == "jdk"
            && result[i]["binary"].has_key("package")
            && result[i]["binary"]["os"] == "windows"
            && arch == result[i]["binary"]["architecture"]
        {
            let file_name = result[i]["binary"]["package"]["name"].to_string();
            let version_name = result[i]["release_name"].to_string();
            let mut version_store = java_ver::read_version();

            if !version_store.contains(&version_name, "Eclipse Adoptium") {
                // println!("{}", file_name);
                let download_url = format!(
                    "https://mirrors.tuna.tsinghua.edu.cn/Adoptium/{}/jdk/{}/windows/{}",
                    version, arch, file_name
                );

                // println!("{}", download_url);
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("temp/");
                let save_location = current_location.to_str().unwrap();
                // println!("{}", save_location);
                let download = Download::new(&download_url, Some(save_location), None);
                match download.download_async().await {
                    Ok(_) => {
                        println!("{}", green.apply_to("下载完成"));
                        let mut java_location = std::env::current_exe().unwrap();
                        java_location.pop();
                        java_location.push("java/");
                        java_location.push(&version_name);
                        let java: java_ver::JavaNew = java_ver::JavaNew::new(
                            "Eclipse Adoptium",
                            &version_name,
                            "Hotspot",
                            "jdk",
                            java_location.to_str().unwrap(),
                        );
                        let mut current_location = std::env::current_exe().unwrap();
                        current_location.pop();
                        current_location.push("java/");
                        let file = String::from(save_location) + &file_name;
                        ziputil::extract(std::path::Path::new(&file), &current_location);
                        java_ver::version_record(java);
                        match std::fs::remove_file(std::path::Path::new(&file)) {
                            Ok(_) => println!("{}", green.apply_to("jdk install finish!")),
                            Err(e) => println!("{} temp file delete failed, {}", red.apply_to("Error"), e.to_string())
                        }
                    }
                    Err(e) => println!("{} Download Failed, {}", red.apply_to("Error"),  e.to_string()),
                };
            } else {
                println!("{}JDK already exist!", red.apply_to("Error"));
            }
        }
        i += 1;
    }
}
