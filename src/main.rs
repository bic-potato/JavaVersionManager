use clap;
pub mod java_remote;
pub mod java_ver;
mod utils;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = clap::App::new("java-version")
        .about("Manage Java Developer Kit Versions")
        .author("ZuoXichen")
        .version("0.1.0")
        .subcommand(
            clap::App::new("list")
                .about("List local available JDKs")
                .short_flag('l')
                .subcommand(
                    clap::App::new("remote").about("List available JDKs from remote server"),
                )
                .subcommand(clap::App::new("local").about("List local jdk versions"))
        )
        .subcommand(
            clap::App::new("enable")
                .about("enable one jdk version globally")
                .arg(clap::Arg::new("version").takes_value(true)),
        )
        .subcommand(
            clap::App::new("get").subcommand(
                clap::App::new("remote").arg(
                    clap::Arg::new("version")
                        .takes_value(true)
                        .value_name("Version")
                        .short('v'),
                ),
            ),
        )
        .get_matches();

    if let Some(f) = matches.subcommand_matches("list") {
        if let Some(_) = f.subcommand_matches("remote") {
            let result = java_remote::list_remote().await;
            println!("{}", result["releases"].pretty(4));
        } else if let Some(_) = f.subcommand_matches("local") {
            let version_list = java_ver::read_version();
            println!("All available JDKs:");
            for element in version_list
            {
                println!("{}", element);
            }
        }
    }else if let Some(g) = matches.subcommand_matches("get"){
        if let Some(r) = g.subcommand_matches("remote") {
            if let Some(v) = r.value_of("version")
            {
                let version = String::from(v);
                let _:i32 = version.trim().parse().expect("Please enter a number");
                java_remote::get_remote(v).await;
            }
            
        }
    } else if let Some(e) = matches.subcommand_matches("enable"){
        if let Some(version) = matches.value_of("version")
        {
            java_ver::enable_version(version);
        }
        
    }
}