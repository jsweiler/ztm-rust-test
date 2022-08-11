// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::{collections::HashMap};

fn main() {
    let mut items = HashMap::new();
    items.insert("chairs", 5);
    items.insert("beds", 3);
    items.insert("tables", 2);
    items.insert("couches", 0);

    let mut total_count: i32 = 0;
    for (name, &val) in items.iter() {
        total_count = total_count + val;
        let stock_count = if val == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", val)
        };
        println!("item={:?}, stock={:?}", name, stock_count);
    }

    println!("total stock count: {:?}", total_count);
}
