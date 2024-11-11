use std::fs;

pub fn update() {
    let current_version = fs::read_to_string("/var/lib/sail/version")
        .ok()
        .map(|s| s.trim().to_string());

    let api_url = "https://api.github.com/repos/jensmeindersma/sail/releases/latest";

    let github = octorust::Client::new(
        String::from("user-agent-name"),
        Credentials::Token(String::from("personal-access-token")),
    );
}
