// main.rs

use std::env;

use search_project_b::search_file;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let query: &str = &args[1];
    let file_path: &str = &args[2];
    let ignore_case = args[0] == "-i";

    match search_file(query, file_path, ignore_case) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
