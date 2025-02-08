use std::slice;

#[link(name = "matrix_ops")]
extern "C" {
    fn multiply_matrices(
        a: *const f64, a_rows: i32, a_cols: i32,
        b: *const f64, b_rows: i32, b_cols: i32,
        result: *mut f64,
    );
}

pub fn matrix_multiply(a: &[f64], a_rows: usize, a_cols: usize, 
                      b: &[f64], b_rows: usize, b_cols: usize) -> Vec<f64> {
    assert_eq!(a_cols, b_rows, "Matrix dimensions must match for multiplication");
    assert_eq!(a.len(), a_rows * a_cols, "Matrix A dimensions don't match data length");
    assert_eq!(b.len(), b_rows * b_cols, "Matrix B dimensions don't match data length");
    
    let mut result = vec![0.0; a_rows * b_cols];
    
    unsafe {
        multiply_matrices(
            a.as_ptr(), a_rows as i32, a_cols as i32,
            b.as_ptr(), b_rows as i32, b_cols as i32,
            result.as_mut_ptr(),
        );
    }
    
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let a = vec![1.0, 2.0, 3.0, 4.0];  // 2x2 matrix
        let b = vec![5.0, 6.0, 7.0, 8.0];  // 2x2 matrix
        
        let result = matrix_multiply(&a, 2, 2, &b, 2, 2);

        assert_eq!(result, vec![19.0, 22.0, 43.0, 50.0]);
    }
}