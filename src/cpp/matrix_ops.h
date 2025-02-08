#pragma once
#include <Eigen/Dense>

extern "C" {
    // Perform matrix multiplication and return result through parameters
    void multiply_matrices(
        const double* a, int a_rows, int a_cols,
        const double* b, int b_rows, int b_cols,
        double* result
    );
}
