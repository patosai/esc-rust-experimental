all:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

readelf:
	arm-none-eabi-readelf -A target/thumbv7em-none-eabi/debug/esc

.PHONY: all clean readelf
