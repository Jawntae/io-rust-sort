use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::person::{Address, Person};
use std::cmp::Ordering;

pub fn read_file(input:&str) -> Result<Vec<Person>, std::io::Error>{

    let file = File::open(input)?;
    let reader = BufReader::new(file);

    //vector of people structs
    let mut people = Vec::new();


    for line in reader.lines() {
        let line = line?;

        //splits line input by comman and collects everything into vector
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        //address struct for Person field

        let address = Address {
            street: parts[2].to_string(),
            city: parts[3].to_string(),
            state: parts[4].to_string(),
            zip: parts[5].to_string(),
        };



        //person struct
        let person = Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            address,
            phone_number: parts[6].to_string()
        };

        people.push(person);
    }


    Ok(people)
}


pub fn write_file(output: &str, people: &[Person]) -> Result<(), std::io::Error> {

    let mut output = File::create(output)?;
    for person in people {
        writeln!(output, "{}, {}, {}, {}, {}, {}, {}",
        person.first_name,
        person.last_name,
        person.address.street,
        person.address.city,
        person.address.state,
        person.address.zip,
        person.phone_number,
    )?
    }

    
    Ok(())
}


//sorts but still needs to ignore case and whitespace
pub fn struct_sort(people: &mut Vec<Person>) {
    let arr_len = people.len();
    let mut min_idx:usize;
    for i in 0..arr_len {
        min_idx = i;
        for j in (i+1)..arr_len {

            //use .cmp, to_lowercase(), and Ordering::Less for comparing values
            // from ascending order
            if people[j].first_name.to_lowercase()
            .cmp(&people[min_idx].first_name.to_lowercase()) == Ordering::Less {
                min_idx = j;
            }
        }

        if min_idx !=i {
            people.swap(i,min_idx);
        }
    }

}