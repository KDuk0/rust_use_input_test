use std::str::FromStr;

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