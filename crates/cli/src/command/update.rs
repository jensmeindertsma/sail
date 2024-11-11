use reqwest::{
    blocking::Client,
    header::{ACCEPT, USER_AGENT},
};
use std::{fs, process::Command};

pub fn update() {
    let current_version = fs::read_to_string("/var/lib/sail/version")
        .ok()
        .map(|s| s.trim().to_owned());

    let api_url = "https://api.github.com/repos/jensmeindertsma/sail/releases/latest";

    let client = Client::new();
    let response = client
        .get(api_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:132.0) Gecko/20100101 Firefox/132.0",
        )
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();
    dbg!(&response["tag_name"]);
    let latest_version = response["tag_name"].as_str().unwrap_or("");
    if current_version.is_some() && current_version.as_deref() == Some(latest_version) {
        println!("Already up-to-date.");
        return;
    }

    let asset_url = response["assets"]
        .as_array()
        .and_then(|assets| {
            assets.iter().find_map(|asset| {
                asset["name"]
                    .as_str()
                    .filter(|name| *name == format!("sail-{}.tar.gz", latest_version))
                    .and_then(|_| {
                        asset["browser_download_url"]
                            .as_str()
                            .map(|url| url.to_string())
                    })
            })
        })
        .ok_or("Asset not found")
        .unwrap();

    let download_path = format!("/tmp/sail-{}.tar.gz", latest_version);
    let mut response = client.get(&asset_url).send().unwrap();
    let mut file = std::fs::File::create(&download_path).unwrap();
    std::io::copy(&mut response, &mut file).unwrap();

    println!("Downloaded newest assets!");

    Command::new("rm")
        .arg("-rf")
        .arg("/tmp/sail")
        .status()
        .unwrap();
    Command::new("mkdir").arg("/tmp/sail").status().unwrap();

    Command::new("tar")
        .args(["-xzf", &download_path, "-C", "/tmp/sail"])
        .status()
        .unwrap();

    //Remove download archive
    Command::new("rm")
        .arg("-f")
        .arg(format!("/tmp/sail-{}.tar.gz", latest_version))
        .status()
        .unwrap();

    Command::new("systemctl")
        .arg("stop")
        .arg("sail")
        .status()
        .unwrap();

    for binary in ["sail", "saild"] {
        Command::new("sudo")
            .arg("mv")
            .arg(format!("/tmp/sail/{binary}"))
            .arg(format!("/usr/local/bin/{binary}"))
            .status()
            .unwrap();
    }

    for file in ["sail.service", "sail.socket"] {
        Command::new("sudo")
            .arg("mv")
            .arg(format!("/tmp/sail/{file}"))
            .arg(format!("/etc/systemd/system/{file}"))
            .status()
            .unwrap();
    }

    //Remove download files
    Command::new("rm")
        .arg("-rf")
        .arg("/tmp/sail")
        .status()
        .unwrap();

    Command::new("sudo")
        .arg("systemctl")
        .arg("daemon-reload")
        .status()
        .unwrap();

    Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg("sail")
        .status()
        .unwrap();

    println!("Writing version");

    Command::new("sudo")
        .arg("echo")
        .arg(latest_version)
        .arg(">")
        .arg("/var/lib/sail/version")
        .status()
        .unwrap();
    println!("Wrote version");

    println!("Sail updated to version {latest_version}.");
}
