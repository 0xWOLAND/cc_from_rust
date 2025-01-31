use rust_calculator::{add_numbers, subtract_numbers, multiply_numbers, divide_numbers};

fn main() {
    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b, add_numbers(a, b));
    println!("{} - {} = {}", a, b, subtract_numbers(a, b));
    println!("{} * {} = {}", a, b, multiply_numbers(a, b));
    println!("{} / {} = {}", a, b, divide_numbers(a, b));
}
