mod update;

use std::env;
use update::update;

fn main() {
    let mut arguments = env::args().skip(1);

    let command = arguments.next().expect("command should be provided");

    match command.as_str() {
        "update" => update(),
        other => panic!("unknown command `{other}`"),
    }
}
