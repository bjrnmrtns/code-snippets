#include "bor.h"

int32_t bor_main(int32_t x)
{
    return x;
}

int32_t bor_sum(const int32_t* const ptr, size_t size)
{
    int32_t sum = 0;
    for(size_t i = 0; i < size; i++) {
        sum += ptr[i];
    }
    return sum;
}

int32_t bor_sum_colors(const color_t* const ptr, size_t size)
{
    int32_t sum = 0;
    for(size_t i = 0; i < size; i++) {
        sum += ptr[i].r + ptr[i].g + ptr[i].b;
    }
    return sum;
}
