use std::ffi::c_int;

mod ffi;

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { ffi::add(a as c_int, b as c_int) }
}

pub fn subtract(a: i32, b: i32) -> i32 {
    unsafe { ffi::subtract(a as c_int, b as c_int) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    unsafe { ffi::multiply(a as c_int, b as c_int) }
}

pub fn divide(a: i32, b: i32) -> f64 {
    unsafe { ffi::divide(a as c_int, b as c_int) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(3, 4), 7);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 5), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5.0);
    }
}
