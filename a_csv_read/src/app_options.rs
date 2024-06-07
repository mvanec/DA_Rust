// app_options.rs
use getopts::Options;
use std::env;

pub struct AppOptions {
    pub file: String
}

pub fn process_options() -> AppOptions {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "The name of the file to be processed", "<file>");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    let file = matches.opt_str("f");

    if let Some(file) = file {
        AppOptions { file }
    } else if !matches.free.is_empty() {
        AppOptions { file: matches.free[0].clone() }
    } else {
        print_usage(&program, opts);
        std::process::exit(1);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
