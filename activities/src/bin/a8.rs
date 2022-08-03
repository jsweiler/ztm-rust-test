// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Strawberry,
    Lemon
}

struct Drink {
    flavor: Flavor,
    ounces: i32
}

fn main() {

    let drink = Drink {
        flavor: Flavor::Strawberry,
        ounces: 16
    };
    print_drink(drink);
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Lemon => println!("drink oz {:?} flavor lemon", drink.ounces),
        Flavor::Strawberry => println!("drink oz {:?} flavor stawberry", drink.ounces),
    }
}
