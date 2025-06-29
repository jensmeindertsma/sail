use std::{
    env,
    process::{ExitCode, Termination},
};

use bollard::{Docker, query_parameters::ListImagesOptions};

#[tokio::main]
async fn main() -> impl Termination {
    let mut arguments = env::args().skip(1);

    match arguments.next() {
        None => show_status(),
        Some(argument) => match argument.as_str().trim() {
            "help" => {
                println!("Help is on the way!")
            }
            "status" => show_status(),
            "list" => {
                let docker = Docker::connect_with_socket_defaults().unwrap();
                let images = docker
                    .list_images(Some(ListImagesOptions::default()))
                    .await
                    .unwrap();
                for image in images {
                    println!("- {:?}", image.repo_tags);
                }
            }
            _ => {
                eprintln!("unknown argument {argument}");
                return ExitCode::FAILURE;
            }
        },
    }

    ExitCode::SUCCESS

    // empty or status
    // help
    // new
    // view <name>
    // deploy <name>
    // restart <name>
    // stop <name>
    // delete <name>
    // rename <name>
    // edit <name> (opens list allowing property to modify to be selected)
    // - domains
    // - fallback page
}

fn show_status() {
    println!("STATUS")
}
