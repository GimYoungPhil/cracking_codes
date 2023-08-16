use std::env;
use std::process;

use cracking_codes::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem pasing arguments: {err}");
        process::exit(1);
    });
    println!("{}", config.query);
    println!("{}", config.file_path);

    if let Err(e) = cracking_codes::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
