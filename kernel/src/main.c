#include <stddef.h>
#include <stdint.h>

#include "roxy_loader.h"

static void kernel_main(const BootInfo *bootinfo);
static void put_pixel(const Framebuffer *framebuffer, size_t x, size_t y, uint32_t color);

void _start(const BootInfo *bootinfo) {
    kernel_main(bootinfo);

    for (;;) {
        __asm__ volatile ("hlt");
    }
}

static void put_pixel(const Framebuffer *framebuffer, size_t x, size_t y, uint32_t color) {
    size_t bytes_per_pixel = framebuffer->size / (framebuffer->stride * framebuffer->height);
    size_t offset = (y * framebuffer->stride + x) * bytes_per_pixel;
    volatile uint8_t *pixel = (volatile uint8_t *)framebuffer->ptr + offset;

    switch (framebuffer->pixel_format) {
        case Rgb:
            pixel[0] = (uint8_t)((color >> 16) & 0xff);
            pixel[1] = (uint8_t)((color >> 8) & 0xff);
            pixel[2] = (uint8_t)(color & 0xff);
            break;
        case Bgr:
            pixel[0] = (uint8_t)(color & 0xff);
            pixel[1] = (uint8_t)((color >> 8) & 0xff);
            pixel[2] = (uint8_t)((color >> 16) & 0xff);
            break;
        default:
            pixel[0] = (uint8_t)(color & 0xff);
            pixel[1] = (uint8_t)((color >> 8) & 0xff);
            pixel[2] = (uint8_t)((color >> 16) & 0xff);
            break;
    }
}

static void kernel_main(const BootInfo *bootinfo) {
    const Framebuffer *framebuffer = &bootinfo->framebuffer;
    uint32_t background = 0x101820;
    uint32_t line = 0xffd166;

    for (size_t y = 0; y < framebuffer->height; y++) {
        for (size_t x = 0; x < framebuffer->width; x++) {
            put_pixel(framebuffer, x, y, background);
        }
    }

    for (size_t x = 0; x < framebuffer->width; x++) {
        size_t y = x * framebuffer->height / framebuffer->width;
        put_pixel(framebuffer, x, y, line);
    }
}
