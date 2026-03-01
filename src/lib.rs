//! main library that calls other modules

pub mod config;
pub mod person;
pub mod io;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file() -> io::Result<()> {
    let file = File::open(Config::input)?;

}