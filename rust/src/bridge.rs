#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("calculator.h");

        fn add(a: i32, b: i32) -> i32; 
        fn subtract(a: i32, b: i32) -> i32;
        fn multiply(a: i32, b: i32) -> i32;
        fn divide(a: i32, b: i32) -> f64;
    }
}

pub use ffi::*;
