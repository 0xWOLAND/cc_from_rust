#include "matrix.h"

#include <Eigen/Dense>
#include <vector>

std::vector<double> multiply_matrices(
    const std::vector<double>& a,
    const std::vector<double>& b,
    int rows_a,
    int cols_a)
{
    // Map a and b onto Eigen matrices
    Eigen::Map<const Eigen::MatrixXd> A(a.data(), rows_a, cols_a);
    Eigen::Map<const Eigen::MatrixXd> B(b.data(), cols_a, rows_a);

    // Perform multiplication
    Eigen::MatrixXd C = A * B;

    // Copy the result back into a std::vector<double>
    std::vector<double> result(C.size());
    Eigen::Map<Eigen::MatrixXd>(result.data(), C.rows(), C.cols()) = C;

    return result;
}
