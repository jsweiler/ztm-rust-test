// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


struct Person {
    age: i32,
    name: String,
    color: String
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn print_color(color: &str) {
    println!("color: {:?}", color);
}

fn main() {
    let people = vec![
        Person{
            age: 10,
            name: "Bob".to_owned(),
            color: "Red".to_owned()
        },
        Person{
            age: 44,
            name: "Jane".to_owned(),
            color: "Red".to_owned()
        },
        Person{
            age: 6,
            name: "Jones".to_owned(),
            color: "Yellow".to_owned()
        }
    ];

    for person  in people {
        if person.age <= 10 {
            print_name(&person.name);
            print_color(&person.color);
        }
    }
}
