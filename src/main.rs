use clap::{ self, Parser, command, Subcommand, arg };
use java_ver::install_local_jdk;
use tokio;
use dotenv::dotenv;
use std::io::Write;

pub mod java_remote;
pub mod java_ver;
pub mod utils;

#[derive(Parser)]
#[command(
    name="jvman",
    author = "BicPotato",
    version = "0.3.0",
    about = "Install and manage JDKs",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[command(subcommand)]
        list_jdk: ListJDK,
    },
    Install {
        #[command(subcommand)]
        install: Install,
    },
    Enable {
        #[arg(short, long)]
        implementor: String,
        #[arg(short='v', long)]
        jdk_version: String,
        #[arg(short, long, default_value_t = false)]
        global: bool,
    },
}

#[derive(Subcommand)]
enum Install {
    Remote {
        #[arg(short = 'v', long)]
        jdk_version: i32,
    },
    Local {
        #[arg(short, long)]
        jdk_path: String,
    },
}

#[derive(Subcommand)]
enum ListJDK {
    Local,
    Remote,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    match cli.command {
        Commands::List { list_jdk } => {
            match list_jdk {
                ListJDK::Local => {
                    let mut version_list = java_ver::read_version();
                    println!("All available JDKs:");
                    let jdk_versions = version_list.get_java_versions();
                    for jdk_version in jdk_versions {
                        println!(
                            "\t{} {}",
                            jdk_version.get_implementor(),
                            jdk_version.get_full_version()
                        );
                    }
                }
                ListJDK::Remote => {
                    let java_release_result = java_remote::list_remote().await;
                    print!("available_lts_releases: ");
                    for version in java_release_result.available_lts_releases {
                        print!("{} ", version);
                    }
                    print!("\n");
                    print!("available_releases: ");
                    for version in java_release_result.available_releases {
                        print!("{} ", version);
                    }
                    print!("\n");
                    std::io::stdout().flush().unwrap();
                }
            }
        }
        Commands::Install { install } => {
            match install {
                Install::Local { jdk_path } => {
                    let postions = jdk_path.replace("\"", "");
                    install_local_jdk(&postions);
                }
                Install::Remote { jdk_version } => {
                    java_remote::get_remote(jdk_version).await;
                }
            }
        }
        Commands::Enable { implementor, jdk_version, global } => {
            if global {
                java_ver::enable_jdk_global(&implementor, &jdk_version);
            } else {
                java_ver::enable_local(&implementor, &jdk_version);
            }
        }
    }
    // let matches = Command::new("java-version")
    //     .about("Manage Java Developer Kit Versions")
    //     .author("ZuoXichen")
    //     .version("0.2.0")
    //     .subcommand(
    //         Command::new("list")
    //             .about("List local available JDKs")
    //             .short_flag('l')
    //             .subcommand(
    //                 Command::new("remote").about("List available JDKs from remote server"),
    //             )
    //             .subcommand(Command::new("local").about("List local jdk versions")),
    //     )
    //     .subcommand(
    //         Command::new("enable")
    //             .about("enable one jdk version globally")
    //             .args(&[Arg::new("implementor").short('i'),
    //             Arg::new("version").short('v'),
    //             Arg::new("global").short('g').long("global").help("enable jdk globally").default_missing_value("true")
    //             ])

    //             .override_usage("jdkman.exe enable -i [IMPLEMENTOR] -v [JDK_VERSION]"),
    //     )
    //     .subcommand(
    //         Command::new("disable")
    //             .about("disable jdk version globally")
    //             .arg(clap::Arg::new("version")),
    //     )
    //     .subcommand(
    //         Command::new("get").subcommand(
    //             Command::new("remote").arg(
    //                 clap::Arg::new("version")
    //                     .value_name("Version"),
    //             )).subcommand(
    //                 Command::new("local").arg(
    //                     clap::Arg::new("path")
    //                         .value_name("Path"),
    //                 ),
    //             ))
    //     .get_matches();

    // if let Some(f) = matches.subcommand_matches("list") {
    //     if let Some(_) = f.subcommand_matches("remote") {
    //         let result = java_remote::list_remote().await;
    //         println!("{:?}", result);
    //     } else if let Some(_) = f.subcommand_matches("local") {
    //         let mut version_list = java_ver::read_version();
    //         println!("All available JDKs:");
    //         let mut element = version_list.get_java_versions();
    //         for mut elem in element {
    //             println!("\t\"{}\" {}", elem.get_implementor(), elem.get_full_version())
    //         }
    //     }
    // } else if let Some(g) = matches.subcommand_matches("get") {
    //     if let Some(r) = g.subcommand_matches("remote") {
    //         if let Some(v) = r.value_of("version") {
    //             let version = String::from(v);
    //             let _: i32 = version.trim().parse().expect("Please enter a number");
    //             java_remote::get_remote(v).await;
    //         }
    //     } else if let Some(local) = g.subcommand_matches("local") {
    //         if let Some(pos) = local.value_of("path") {
    //             let postion = String::from(pos);
    //             let postions = postion.replace("\"", "");
    //             read_local(&postions);
    //         }
    //     }
    // } else if let Some(e) = matches.subcommand_matches("enable") {
    //     if let Some(implementor) = e.value_of("implementor") {
    //         if let Some(version) = e.value_of("version") {
    //             let implementors = String::from(implementor);
    //             let imple = implementors.replace("\"", "");
    //             java_ver::enable_version(&imple, version);
    //             // if let Some(_) = e.value_of("global"){
    //             //     java_ver::enable_version(&imple, version);
    //             // } else {
    //             //     java_ver::enable_temp(&imple, version);
    //             // }
    //         }
    //     }
    // } else if let Some(_) = matches.subcommand_matches("disable") {
    //     let mut current_location = std::env::current_exe().unwrap();
    //     current_location.pop();
    //     current_location.push("OpenJDK/");
    //     let result = std::fs::remove_dir_all(&current_location);
    //     match result {
    //         Ok(_) => {
    //             let green = Style::new().green();
    //             println!("{}", green.apply_to("Disable JDK SUCCESS"))
    //         }
    //         Err(e) => {
    //             let red = Style::new().red();
    //             println!("{} {}", red.apply_to("Disable JDK FAILED"), e.to_string());
    //         }
    //     }
    // }
}