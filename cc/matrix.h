#ifndef MATRIX_H_
#define MATRIX_H_

#include <vector>


std::vector<double> multiply_matrices(
    const std::vector<double>& a,
    const std::vector<double>& b,
    int rows_a,
    int cols_a); // We'll assume b has size cols_a x rows_a

#endif  // MATRIX_H_
