// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    qty: i32
}

fn display_qty(item: &GroceryItem) {
    println!("Qty: {:?}", item.qty);
}

fn display_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id);
}

fn main() {
    let item = GroceryItem{
        id: 2,
        qty: 43
    };
    display_id(&item);
    display_qty(&item);
}
