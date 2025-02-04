#pragma once

#include <cstdint>
#include <stdexcept>

extern "C" {  // <-- Add this to make the functions accessible globally
    int32_t add(int32_t a, int32_t b);
    int32_t subtract(int32_t a, int32_t b);
    int32_t multiply(int32_t a, int32_t b);
    double divide(int32_t a, int32_t b);
}
