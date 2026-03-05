//! main library that calls other modules

pub mod config;
pub mod person;
pub mod io;


use config::Config;
//use person::Person;

//main driver for the program
pub fn run(config:Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut people = io::read_file(&config.input)?;
    //println!("people: {:?}", people);
    for element in &people {
        println!("{:?}: {:?}", element.first_name, element.last_name);
    }

    //let sorted_people = io::struct_sort(&mut people);

    io::struct_sort(&mut people);
    println!("sorted version\n");
    for element in &people {
        println!("{:?}: {:?}", element.first_name, element.last_name);
    }


    
    Ok(())
}