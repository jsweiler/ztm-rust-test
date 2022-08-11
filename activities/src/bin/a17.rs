// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let my_string = "ThisIsAString".to_owned();

    let upper = my_string.to_uppercase();
    print!("upper: {:?}", upper);

    let lower = my_string.to_lowercase();
    print!("lower: {:?}", lower);
    
}
