use crate::java_ver;
use download_rs::sync_download::Download;
use json;
use reqwest;

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
    println!("{}", url);
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
        //println!("{}",result[i]["binary"].pretty(4));
        if result[i]["binary"]["image_type"] == "jdk"
            && result[i]["binary"].has_key("package")
            && result[i]["binary"]["os"] == "windows"
            && arch == result[i]["binary"]["architecture"]
        {
            let file_name = result[i]["binary"]["package"]["name"].to_string();
            let version_name = result[i]["release_name"].to_string();
            let version_list = java_ver::read_version();
            if !version_list.contains(&version_name) {
                println!("{}", file_name);
                let download_url = format!(
                    "https://mirrors.tuna.tsinghua.edu.cn/AdoptOpenJDK/{}/jdk/x64/windows/{}",
                    version, file_name
                );

                println!("{}", download_url);
                let mut current_location = std::env::current_exe().unwrap();
                current_location.pop();
                current_location.push("temp/");
                let save_location = current_location.to_str().unwrap();
                println!("{}", save_location);
                let download = Download::new(&download_url, Some(save_location), None);
                match download.download() {
                    Ok(_) => {
                        println!("下载完成");
                        let mut java_location = std::env::current_exe().unwrap();
                        java_location.pop();
                        java_location.push("java/");
                        java_location.push(&version_name);
                        let java: java_ver::Java = java_ver::Java::new(
                            version_name,
                            String::from("Hotspot"),
                            String::from("jdk"),
                            String::from(java_location.to_str().unwrap()),
                        );
                        let file = String::from(save_location) + &file_name;
                        java_ver::version_record(std::path::Path::new(&file), java)
                    }
                    Err(e) => println!("下载出错：{}", e.to_string()),
                };
            }
        }
        i += 1;
    }
}
