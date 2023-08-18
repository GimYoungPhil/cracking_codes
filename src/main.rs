use std::env;
use std::process;

use cracking_codes;

fn main() {
    let config = cracking_codes::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = cracking_codes::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
