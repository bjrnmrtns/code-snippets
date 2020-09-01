#include <string>
#include <vector>
#include <png.h>

namespace Png {
class Handle
{
    public:
        Handle()
        : png(png_create_write_struct(PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr))
        , info(png_create_info_struct(png))
        {
        }
        ~Handle() {
            png_free_data(png, info, PNG_FREE_ALL, -1);
            png_destroy_write_struct(&png, static_cast<png_infopp>(nullptr));
        }
        png_structp png;
        png_infop info;
};

void WriteImage(Handle& handle, std::vector<unsigned char>& data, size_t width, size_t height, std::vector<std::pair<std::string, std::string>>& text_key_values)
{
    FILE *fp = fopen("test.png", "wb");
    std::vector<png_bytep> rows(height, nullptr);
    for(auto y = 0; y < width; y++) {
        rows[y] = &data[y * width];
    }

    std::vector<png_text> text_data;
    for(auto& key_value: text_key_values)
    {
        png_text t;
        t.key = &key_value.first[0];
        t.text = &key_value.second[0];
        t.compression = PNG_TEXT_COMPRESSION_zTXt;
        text_data.push_back(t);
    }
    png_set_text(handle.png, handle.info, text_data.data(), text_data.size());
    png_set_IHDR(handle.png, handle.info, width, height, 8, PNG_COLOR_TYPE_GRAY, PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_BASE, PNG_FILTER_TYPE_BASE);
    
    png_init_io(handle.png, fp);
    png_set_rows(handle.png, handle.info, rows.data());
    png_write_png(handle.png, handle.info, PNG_TRANSFORM_IDENTITY, nullptr);
    fclose(fp);
}

}

int main()
{
    size_t size = 128;
    std::vector<png_byte> raw_gray(size * size, 127);
    Png::Handle png_handle;

    std::vector<std::pair<std::string, std::string>> key_values;
    key_values.push_back(std::make_pair("B", "42"));
    key_values.push_back(std::make_pair("R", "46"));
    Png::WriteImage(png_handle, raw_gray, size, size, key_values);

    return 0;
}
