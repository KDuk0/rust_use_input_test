use crate::utils::{ask_for_a_number, ask_for_a_string};
use std::fs::{write, read_to_string};
use std::str::FromStr;

mod utils;

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn print(&self) {
        println!("{} {} {} old", &self.first_name, &self.last_name, &self.age);
    }
}

fn main() {

    let first_name = ask_for_a_string("What is your first name?");
    let last_name = ask_for_a_string("What is your last name?");
    let age = ask_for_a_number("How old are you?").unwrap_or(0);
    let person = Person {
        first_name,
        last_name,
        age,
    };
  
    match write_person(&person) {
        Ok(_) => println!("people.txt was written successfully"),
        Err(err) => println!("There was error while writing people.txt: {}", err),
    }

    match read_person() {
        Ok(person) => {
            println!("people.txt was read successfully:");
            person.print();
        },
        Err(err) => println!("There was error while reading people.txt: {}", err),
    }
}

//adding new function that writes user input to .txt file; use push_str to add user input into string
//std::io:Result<()> is the output we are returning
fn write_person(person: &Person) -> std::io::Result<()> {
    let mut output = String::new();
    output.push_str(&person.first_name);
    output.push('\n');
    output.push_str(&person.last_name);
    output.push('\n');
    output.push_str(&person.age.to_string());
    output.push('\n');
    write("people.txt", output)
}

fn read_person() -> Result<Person, std::io::Error> {
    let input = read_to_string("people.txt")?;
    let mut lines = input.split('\n');
    let first_name = lines.next().unwrap_or("").to_string();
    let last_name = lines.next().unwrap_or("").to_string();
    let age_as_string = lines.next().unwrap_or("0").to_string();
    let age = u8::from_str(&age_as_string).unwrap_or(0);
    let person = Person {
        first_name,
        last_name,
        age,
    };
    Ok(person)
}