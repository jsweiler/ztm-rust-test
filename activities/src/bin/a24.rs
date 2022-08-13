// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

use std::vec;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let num: i32 = 10;
    let tripled: Vec<_> = data
        .iter()
        .map(|num| num * 3)
        .filter(|a| a > &num)
        .collect();

        for num in tripled {
            println!("num: {:?}", num);
        }
}
