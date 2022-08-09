// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color{
    Red,
    Blue
}

impl Color{
    fn print(&self) {
        match &self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red")
        }
    }
}

struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: f64,
    color: Color
}

impl ShippingBox {

    fn create() -> Self {
        Self { dimensions: (3,2,3), color: Color::Red, weight: 33.0}
    }

    fn show_box(&self) {
        self.color.print();
        println!("{:?} pounds, dimensions{:?}", self.weight, self.dimensions);
    }
}

fn main() {
    let box_ship = ShippingBox { 
        dimensions: (2,2,2), 
        weight: 34.3,
        color: Color::Blue
    };
    box_ship.show_box();

    let new_box = ShippingBox::create();
    new_box.show_box();
}
