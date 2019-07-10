all: default

default:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

readelf: all
	arm-none-eabi-readelf -A target/thumbv7em-none-eabi/debug/esc

.PHONY: all default release clean readelf
