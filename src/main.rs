use clap;
use tokio;
use console::Style;
use dotenv::dotenv;
use crate::java_ver::read_local;

pub mod java_remote;
pub mod java_ver;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = clap::App::new("java-version")
        .about("Manage Java Developer Kit Versions")
        .author("ZuoXichen")
        .version("0.2.0")
        .subcommand(
            clap::App::new("list")
                .about("List local available JDKs")
                .short_flag('l')
                .subcommand(
                    clap::App::new("remote").about("List available JDKs from remote server"),
                )
                .subcommand(clap::App::new("local").about("List local jdk versions")),
        )
        .subcommand(
            clap::App::new("enable")
                .about("enable one jdk version globally")
                .args(&[clap::Arg::new("implementor").takes_value(true).short('i'), 
                clap::Arg::new("version").takes_value(true).short('v'),
                // clap::Arg::new("global").short('g').long("global").takes_value(false).help("enable jdk globally").default_missing_value("true")
                ])
                
                .override_usage("jdkman.exe enable -i [IMPLEMENTOR] -v [JDK_VERSION]"),
        )
        .subcommand(
            clap::App::new("disable")
                .about("disable jdk version globally")
                .arg(clap::Arg::new("version").takes_value(true)),
        )
        .subcommand(
            clap::App::new("get").subcommand(
                clap::App::new("remote").arg(
                    clap::Arg::new("version")
                        .takes_value(true)
                        .value_name("Version"),
                )).subcommand(
                    clap::App::new("local").arg(
                        clap::Arg::new("path")
                            .takes_value(true)
                            .value_name("Path"),
                    ),
                ))
        .get_matches();

    if let Some(f) = matches.subcommand_matches("list") {
        if let Some(_) = f.subcommand_matches("remote") {
            let result = java_remote::list_remote().await;
            println!("{}", result.pretty(4));
        } else if let Some(_) = f.subcommand_matches("local") {
            let mut version_list = java_ver::read_version();
            println!("All available JDKs:");
            let mut element = version_list.get_java_versions();
            for mut elem in element {
                println!("\t\"{}\" {}", elem.get_implementor(), elem.get_full_version())
            }
        }
    } else if let Some(g) = matches.subcommand_matches("get") {
        if let Some(r) = g.subcommand_matches("remote") {
            if let Some(v) = r.value_of("version") {
                let version = String::from(v);
                let _: i32 = version.trim().parse().expect("Please enter a number");
                java_remote::get_remote(v).await;
            }
        } else if let Some(local) = g.subcommand_matches("local") {
            if let Some(pos) = local.value_of("path") {
                let postion = String::from(pos);
                let postions = postion.replace("\"", "");
                read_local(&postions);
            }
        }
    } else if let Some(e) = matches.subcommand_matches("enable") {
        if let Some(implementor) = e.value_of("implementor") {
            if let Some(version) = e.value_of("version") {
                let implementors = String::from(implementor);
                let imple = implementors.replace("\"", "");
                java_ver::enable_version(&imple, version);
                // if let Some(_) = e.value_of("global"){
                //     java_ver::enable_version(&imple, version);
                // } else {
                //     java_ver::enable_temp(&imple, version);
                // }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("disable") {
        let mut current_location = std::env::current_exe().unwrap();
        current_location.pop();
        current_location.push("OpenJDK/");
        let result = std::fs::remove_dir_all(&current_location);
        match result {
            Ok(_) => {
                let green = Style::new().green();
                println!("{}", green.apply_to("Disable JDK SUCCESS"))
            }
            Err(e) => {
                let red = Style::new().red();
                println!("{} {}", red.apply_to("Disable JDK FAILED"), e.to_string());
            }
        }
    }
}
