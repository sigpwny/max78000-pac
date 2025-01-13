TARGET=max78000

all: patch generate format

# Patch the SVD file
patch:
	svdtools patch svd/$(TARGET).yaml svd/$(TARGET).svd.patched

# Generate Rust code from the SVD file
generate:
	svd2rust --target=cortex-m --reexport-core-peripherals -i svd/$(TARGET).svd.patched -o .
	form -i lib.rs -o src/
	rm lib.rs

# Format the generated code
format:
	cargo fmt --all

.PHONY: all patch generate format
