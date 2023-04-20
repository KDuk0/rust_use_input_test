use std::fs::{read_to_string, write};
use std::str::{FromStr, Split};

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}
 impl Person {
    
    pub fn new() -> Self {
        let first_name = ask_for_a_string("What is your first name?");
        let last_name = ask_for_a_string("What is your last name?");
        let age = ask_for_a_number("How old are you?").unwrap_or(0);
        Person {
            first_name,
            last_name,
            age,
        }
    }
}

pub fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}

pub fn read_number() -> Result<u8, String> {
    let input = read_string();
    
    if input.is_empty() {
        Err("You didn't enter any data".to_string())
    }
    else {
        u8::from_str(&input).or(Err("You've entered an invalid number. Please enter a value between 0 and 255".to_string()))
        // I used this but it messes up the input:
        //read_string().parse().or(Err("You've entered an invalid number. Please enter a value between 0 and 255".to_string()))
    }
         

}

fn ask(question: &str) {
    println!("{}", question);
}

pub fn ask_for_a_number(question: &str) -> Result<u8, String> {
   ask(question);
   read_number()
}

pub fn ask_for_a_string(question: &str) -> String {
    ask(question);
    read_string()
}



pub fn read_person(lines: &mut Split<char>) -> Option<Person> {
    
    let first_name = lines.next()?;
    let last_name = lines.next()?;
    let age_as_string = lines.next()?;
    let age = u8::from_str(&age_as_string).unwrap_or(0);
    let person = Person {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age,
    };
    Some(person)
}

    
pub fn read_people() -> Result<Vec<Person>, std::io::Error> {
    let input = read_to_string("people.txt")?;
    let mut lines = input.split('\n');
    let mut people = vec![];
    while let Some(person) = read_person(&mut lines) {
        people.push(person)
    }
    Ok(people)
}

//adding new function that writes user input to .txt file; use push_str to add user input into string
//std::io:Result<()> is the output we are returning
pub fn write_people(people: Vec<Person>) -> std::io::Result<()> {
    let mut output = String::new();

    for person in people {
    output.push_str(&person.first_name);
    output.push('\n');
    output.push_str(&person.last_name);
    output.push('\n');
    output.push_str(&person.age.to_string());
    output.push('\n');
     }
    write("people.txt", output)
}