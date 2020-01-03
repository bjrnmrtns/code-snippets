#include <SDL2/SDL.h>

struct color_t {
    unsigned char r, g, b, a;
};

int main()
{
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window* window = SDL_CreateWindow("framebuffer-test", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 480, 0);
    SDL_Surface* screen = SDL_GetWindowSurface(window);
    color_t colors[640 * 480];
    SDL_Surface* framebuffer = SDL_CreateRGBSurfaceFrom(&colors, 640, 480, 32, 4 * 640, 0x000000ff, 0x0000ff00, 0x00ff0000, 0xff000000);
    for(size_t i = 0; i < 640 * 240; i++) {
        colors[i].r = 255;
        colors[i].g = 0;
        colors[i].b = 0;
        colors[i].a = 255;
    }
    for(size_t i = 640 * 240; i < 640 * 480; i++) {
        colors[i].r = 0;
        colors[i].g = 0;
        colors[i].b = 255;
        colors[i].a = 255;
    }
    SDL_BlitSurface(framebuffer, NULL, screen, NULL);
    SDL_FreeSurface(framebuffer);
    SDL_UpdateWindowSurface(window);
    SDL_Delay(2000);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return 0;
}
