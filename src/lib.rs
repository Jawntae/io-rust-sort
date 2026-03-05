//! main library that calls other modules

pub mod config;
pub mod person;
pub mod io;


use config::Config;
//use person::Person;

//main driver for the program
pub fn run(config:Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut people = io::read_file(&config.input)?;


    //unsorted print test
    for element in &people {
        println!("{:?}: {:?}", element.first_name, element.last_name);
        
    }

    println!("\n");
    
    //sorted print test
    io::struct_sort(&mut people);
    println!("===== sorted version =====\n");
    for element in &people {
        println!("{:?}: {:?}", element.first_name, element.last_name);
    }

    io::write_file(&config.output, &people)?;


    
    Ok(())
}