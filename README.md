# Fractal OS

An operating system written in Rust.

 - [Build](#build)
    - [Creating a bootable image](#creating-a-bootable-image)
    - [Running](#running)
 - [Implementation details](#implementation-details)
 - [About](#about)
 - [Things to add](#things-to-add)

## Build

To build the OS, make sure to use `nightly` rust build.
This is required due to the usage of some unstable flags and properties that are
only available in the `nightly` build.

> Note: the target has been specified in the `.cargo/config.toml`.
```shell script
cargo build
```

> Note: if compiling for custom a target, you can override the target using the `--target <target>` flag.
```shell script
cargo build --target x86_64-fractal_os.json
```

### Creating a bootable image

To create a bootable image, we are using bootloader dependency. Alongside, we will
be using `bootimage` tool to compile the kernel and bootloader and link them together.

Make sure you have bootimage installed.

```shell script
cargo install bootimage
```

> Note: to run bootimage and building the bootloader, you need to have the `llvm-tools-preview`
> rustup component installed.
```shell script
rustup component add llvm-tools-preview
```

After the dependencies and tools have been installed, you can run cargo bootimage to create a 
bootable disk image.
```shell script
cargo bootimage
```

### Running

After generating a bootable image file, you can run Fractal easily on an emulator such as QEMU.

```shell script
qemu-system-x86_64 -drive format=raw,file=target/x86_64-fractal_os/debug/bootimage-fractal_os.bin
```

You can also run it with a cargo runner with the help of bootimage runner.

```shell script
cargo run
```

The above will point to the last build of the bootable image and will start an instance of QEMU.
You can also provide specific target and pass through QEMU options.

```shell script
cargo run --target <target> -- [QEMU options]
```

Everything after the `--` will be passed to QEMU.

### Testing

To run the custom test framework for FractalOS, you can run the cargo test command.

```shell script
cargo test
```

This will run all the unit and integration tests.

> Note: the integration tests are located under `/tests` directory, whilst unit tests
> are self-contained in the `/src` concrete implementation files.

## Implementation Details

The kernel is using the `compiler_builtins` crate along with its `mem` implementation.
The functions however are quite unoptimized. There is currently an [open PR against this issue](https://github.com/rust-lang/compiler-builtins/pull/365),
so either this needs to use our own implementation of functions such as `memcmp` and `memcpy` or wait till this
PR gets merged in.
 
## About

As a software engineer with a background in EEE (Electric and Electronic Engineering), I
always love to find out what makes things tick, and what better way to find out how operating
systems work together with all the hardware, and the software is other than writing your own?

At the same time of exploring what makes all the elements of a clean operating system, I am
also learning Rust. I find it very enjoyable.

## Things to add

 - [ ] Tests
 - [ ] Graphics
 - [ ] Networking
 - [ ] File System
 - [ ] Sound Interface
 - [ ] Applications
 - [ ] Terminal
 
These are just some essentials that are on their way to be implemented. Not in order.
