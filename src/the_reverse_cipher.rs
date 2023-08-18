

pub fn encrypt_message(contents: &str) -> Vec<&str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        results.push(line);
    }

    results
}

pub fn decrypt_message(contents: &str) -> Vec<&str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        results.push(line);
    }

    results
}
