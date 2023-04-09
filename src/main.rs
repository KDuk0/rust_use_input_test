use crate::utils::ask_for_a_number;

mod utils;

fn main() {
   
    match ask_for_a_number("How old are you?") {
     Ok(age) => println!("You are {} old", age),
     Err(err) => println!("{}", err),
    }
}
