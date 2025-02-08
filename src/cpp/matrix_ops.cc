#include "matrix_ops.h"
#include <Eigen/Dense>

using namespace Eigen;

extern "C" {
    void multiply_matrices(
        const double* a, int a_rows, int a_cols,
        const double* b, int b_rows, int b_cols,
        double* result
    ) {
        // Map raw arrays to Eigen matrices
        Map<const Matrix<double, Dynamic, Dynamic, RowMajor>> mat_a(a, a_rows, a_cols);
        Map<const Matrix<double, Dynamic, Dynamic, RowMajor>> mat_b(b, b_rows, b_cols);
        Map<Matrix<double, Dynamic, Dynamic, RowMajor>> mat_result(result, a_rows, b_cols);
        
        // Perform multiplication
        mat_result = mat_a * mat_b;
    }
}