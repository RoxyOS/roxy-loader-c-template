# roxy-loader-c-template

Minimal C kernel template using `roxy-loader` as the bootloader.

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

To run the kernel:

```bash
make run
```
