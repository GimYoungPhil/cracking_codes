use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("{}", query);
    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Shoud have been able to read the file");

    print!("{}", contents);
}
