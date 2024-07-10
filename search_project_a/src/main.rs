use std::{env, process};

use search_project_a::search_file;

fn main() {
    let query: String;
    let file_path: String;
    let ignore_option: &str = "-i";
    let mut ignore_case = false;

    // Args is an iterator. Bypass the first element, which will always
    // be the program name
    let mut args = env::args();
    args.next();

    // First argument could be '-i' or a search string
    let maybe_ignore = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("No arguments provided!");
            process::exit(1);
        }
    };

    // If the first arg is "-i", go ahead and grab the next argument
    // as the search string. We have to clone here due to lifetime limits
    if maybe_ignore == ignore_option {
        ignore_case = true;

        let search_str = match args.next() {
            Some(arg) => arg,
            None => {
                eprintln!("Missing the search string argument!");
                process::exit(1);
            }
        };
        query = search_str.clone();
    } else {
        // Otherwise, the first argument is the search string
        query = maybe_ignore;
    }

    // The code above will have handled both "-i <search>" and "<search>",
    // so this argument has to be the file we're looking in
    file_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Missing the file argument!");
            process::exit(1);
        }
    };

    // Perform the search, or print an error for a file error
    if let Err(err) = search_file(query.as_str(), file_path.as_str(), ignore_case) {
        eprintln!("Error searching for string: {err}");
    }
}
