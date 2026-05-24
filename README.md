# roxy-loader-c-template

Minimal C kernel template using `roxy-loader` as the bootloader.

When the kernel boots successfully, it draws a white line on the screen.

## Requirements

Required tools:

- Rust nightly
- `x86_64-elf-gcc`
- `qemu-system-x86_64`

If you use Nix, enter the dev shell with:

```bash
nix develop
```

## Usage

Commands:

- `make build` - Build the kernel image into `build/`
- `make run` - Build the kernel and run it in QEMU
- `make check` - Check that required host tools are installed
- `make fetch-header` - Download `roxy_loader.h` into `build/include/`
- `make clean` - Remove Cargo build output for `xtask` and delete `build/`

To run the kernel in QEMU:

```bash
make run
```
