
//! Main entry point for program
use std::process;
use std:: {env, fs};



fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


}
