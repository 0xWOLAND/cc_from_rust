extern crate libc;
use libc::c_int;

#[link(name = "cc_calculator", kind = "static")]
extern "C" {
    pub fn add(a: c_int, b: c_int) -> c_int;
    pub fn subtract(a: c_int, b: c_int) -> c_int;
    pub fn multiply(a: c_int, b: c_int) -> c_int;
    pub fn divide(a: c_int, b: c_int) -> f64;
}

pub fn rust_add(a: i32, b: i32) -> i32 {
    unsafe { add(a as c_int, b as c_int) }
}

pub fn rust_subtract(a: i32, b: i32) -> i32 {
    unsafe { subtract(a as c_int, b as c_int) }
}

pub fn rust_multiply(a: i32, b: i32) -> i32 {
    unsafe { multiply(a as c_int, b as c_int) }
}

pub fn rust_divide(a: i32, b: i32) -> f64 {
    unsafe { divide(a as c_int, b as c_int) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(rust_add(3, 4), 7);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(rust_subtract(10, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(rust_multiply(3, 4), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(rust_divide(10, 2), 5.0);
    }
}
