#include <vector>
#include <png.h>

int main()
{
    size_t size = 128;
    std::vector<png_byte> raw_gray(size * size, 127);
    FILE *fp = fopen("test.png", "wb");
    png_structp png_ptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr);
    png_infop info_ptr = png_create_info_struct(png_ptr);
    
    png_init_io(png_ptr, fp);
    png_set_IHDR(png_ptr, info_ptr, size, size, 8, PNG_COLOR_TYPE_GRAY, PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_BASE, PNG_FILTER_TYPE_BASE);
    png_write_info(png_ptr, info_ptr);

    std::vector<png_bytep> rows(size, nullptr);
    for(auto y = 0; y < size; y++) {
        rows[y] = &raw_gray[y * size];
    }
    png_write_image(png_ptr, rows.data());

    png_free_data(png_ptr, info_ptr, PNG_FREE_ALL, -1);
    png_destroy_write_struct(&png_ptr, static_cast<png_infopp>(nullptr));
    fclose(fp);
    return 0;
}
