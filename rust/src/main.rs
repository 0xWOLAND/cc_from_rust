mod bridge;

fn main() {
    let a = 10;
    let b = 2;

    println!("{} + {} = {}", a, b, bridge::add(a, b));
    println!("{} - {} = {}", a, b, bridge::subtract(a, b));
    println!("{} * {} = {}", a, b, bridge::multiply(a, b));
    match bridge::divide(a, b) {
        val => println!("{} / {} = {}", a, b, val),
    }
}
