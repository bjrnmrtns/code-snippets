#pragma once
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    unsigned char r;
    unsigned char g;
    unsigned char b;
} color_t;

int32_t bor_main(int32_t x);
int32_t bor_sum(const int32_t* const ptr, size_t size);
int32_t bor_sum_colors(const color_t* const ptr, size_t size);

#ifdef __cplusplus
}
#endif
