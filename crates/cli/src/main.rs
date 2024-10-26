mod uninstall;
mod update;

use std::{env, process::Termination};
use uninstall::uninstall;
use update::update;

fn main() -> impl Termination {
    let mut arguments = env::args().skip(1);

    let command = arguments.next().expect("command should be provided");

    match command.as_str() {
        "uninstall" => uninstall(),
        "update" => update(),

        other => panic!("unknown command `{other}`"),
    }
}
