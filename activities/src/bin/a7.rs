// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum MyColor {
    Red,
    White, 
    Blue
}

fn main() {
    let color = MyColor::Blue;
    print_color(color);
}

fn print_color(color: MyColor) {
    match color {
        MyColor::Blue => println!("blue"),
        MyColor::Red => println!("red"),
        MyColor::White => println!("white")
    }
}
