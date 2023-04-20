use crate::utils::{read_people, write_people, Person};

mod utils;

fn main() {

    let person = Person::new();
    let people = vec![person];
      
    match write_people(people) {
        Ok(_) => println!("people.txt was written successfully"),
        Err(err) => println!("There was error while writing people.txt: {}", err),
    }

    match read_people() {
        Ok(_people) => println!("people.txt was read successfully:"),
        Err(err) => println!("There was error while reading people.txt: {}", err),
    }
}

