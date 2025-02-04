#include "calculator.h"

int32_t add(int32_t a, int32_t b) {
    return a + b;
}

int32_t subtract(int32_t a, int32_t b) {
    return a - b;
}

int32_t multiply(int32_t a, int32_t b) {
    return a * b;
}

double divide(int32_t a, int32_t b) {
    if (b == 0) {
        throw std::runtime_error("Division by zero");
    }
    return static_cast<double>(a) / b;
}
