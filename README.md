# max78000-pac
This is a [Peripheral Access Crate](https://docs.rust-embedded.org/book/start/registers.html) for Analog Device's [MAX78000](https://www.analog.com/products/max78000.html) microcontroller.

This crate is automatically generated from the SVD file in [Analog Device's MSDK](https://github.com/analogdevicesinc/msdk). [svd2rust](https://github.com/rust-embedded/svd2rust) is used to automatically generate the PAC from the SVD file. The provided SVD file also has a few mistakes, which are patched using [svdtools](https://github.com/rust-embedded/svdtools).

## Building

> [!NOTE]
> This section is only relevant if you need to rebuild the crate. Generated code is already included in the repository.

First, make sure you have Rust installed (via [rustup](https://rustup.rs/)). Then, install the following development dependencies.

```bash
cargo install svd2rust svdtools form
rustup component add --toolchain nightly rustfmt
```

Simply run `make` to generate the crate.

```bash
make
```

## License
This crate is licensed under the [Apache-2.0](./LICENSE) license.

Copyright 2025 SIGPwny.  
Copyright 2025 Analog Devices, Inc.
