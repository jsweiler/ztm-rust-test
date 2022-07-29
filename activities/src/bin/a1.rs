// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let first_name = first_name();
    let last_name = last_name();
    println!("Hello {first_name} {last_name}");
}

fn first_name() -> String {
    String::from("Jeff")
}

fn last_name() -> String {
    String::from("Weiler")
}