
//! Main entry point for program
use std::process;
use std:: {env};
use io_rust_sort::config::Config;
use io_rust_sort::run;



fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprint!("Application error: {}", e);
    }


}
