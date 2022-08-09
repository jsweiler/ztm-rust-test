// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}

fn main() {
    let back_stage = Ticket::Backstage(5000, "Bob".to_owned());
    let vip = Ticket::Vip(3000, "Jane".to_owned());
    let standard = Ticket::Standard(750);

    let tickets = vec![back_stage, vip, standard];

    for ticket in tickets {
        match ticket{
            Ticket::Backstage(price, name) => println!("backstage, price: {:?}, name: {:?}", price, name),
            Ticket::Vip(price, name) => println!("vip, price: {:?}, name: {:?}", price, name),
            Ticket::Standard(price) => print!("standard, price:{:?}", price)
        }
    }
}
