// rust_cxx_demo/src/lib.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // We can include the header for clarity if cxx
        // needs the definitions (not always strictly required):
        include!("matrix.h");

        // The function signature must match exactly:
        fn multiply_matrices(
            a: &Vec<f64>,
            b: &Vec<f64>,
            rows_a: i32,
            cols_a: i32
        ) -> Vec<f64>;
    }
}

pub fn multiply_rust(a: &[f64], b: &[f64], rows_a: i32, cols_a: i32) -> Vec<f64> {
    ffi::multiply_matrices(a, b, rows_a, cols_a)
}

// Optional test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_mult() {
        let rows = 2;
        let cols = 3;
        let a = vec![1.0, 2.0, 3.0,
                     4.0, 5.0, 6.0];
        let b = vec![7.0,  8.0,
                     9.0,  10.0,
                     11.0, 12.0];

        let c = multiply_rust(&a, &b, rows, cols);
        // Should be 2x2 => [58, 64, 139, 154]
        assert_eq!(c, vec![58.0, 64.0, 139.0, 154.0]);
    }
}
