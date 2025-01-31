#[link(name = "calculator", kind = "static")]
extern "C" {
    pub fn add(a: i32, b: i32) -> i32;
    pub fn subtract(a: i32, b: i32) -> i32;
    pub fn multiply(a: i32, b: i32) -> i32;
    pub fn divide(a: i32, b: i32) -> f64;
}

pub fn add_numbers(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

pub fn subtract_numbers(a: i32, b: i32) -> i32 {
    unsafe { subtract(a, b) }
}

pub fn multiply_numbers(a: i32, b: i32) -> i32 {
    unsafe { multiply(a, b) }
}

pub fn divide_numbers(a: i32, b: i32) -> f64 {
    unsafe { divide(a, b) }
}

fn main() {
    println!("works");
}
