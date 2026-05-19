#include <stddef.h>
#include <stdint.h>

#include "roxy_loader.h"

static void kernel_main(const BootInfo *bootinfo);

void _start(const BootInfo *bootinfo) {
    kernel_main(bootinfo);

    for (;;) {
        __asm__ volatile ("hlt");
    }
}

static void kernel_main(const BootInfo *bootinfo) {
    volatile uint8_t *framebuffer = (volatile uint8_t *)bootinfo->framebuffer.ptr;

    for (size_t i = 0; i < bootinfo->framebuffer.size; i++) {
        framebuffer[i] = 69;
    }
}
