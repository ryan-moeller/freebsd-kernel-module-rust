## FreeBSD kernel module in Rust

This repo is mostly an updated version of https://github.com/johalun/echo

It has been updated to Rust 2024 with new bindings to the kernel headers and
tested with Rust version `1.87.0-nightly (794c12416 2025-02-21)`

For more information, see the [accompanying blog post](https://research.nccgroup.com/2022/08/31/writing-freebsd-kernel-modules-in-rust/).

### Setup
* Install libclang:
```bash
sudo pkg install llvm19
```
* Install Rust via Rustup
* `rustup component add rust-src`
* Generate the kernel bindings:
```bash
cargo build -p kernel-sys --target x86_64-unknown-freebsd
```

### Run

```bash
./build.sh
sudo make load
echo "hi rust" > /dev/rustmodule
cat /dev/rustmodule
sudo make unload
```

### Licence
This source code is provided under the terms of the [BSD 2-Clause licence](LICENSE.txt)
and is based on [public-domain work](https://github.com/johalun/echo) by Johannes Lundberg.
