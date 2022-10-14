// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let num = 400;
    let is_big = num > 100;
    let mut message = "";
    match is_big {
        true => {
            message = "its big"
        },
        false => {
            message = "its small"
        }
    }
    println!("this is a test: {:?}", message);
}

