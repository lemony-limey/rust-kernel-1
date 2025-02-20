make: src/main.rs
	cargo bootimage
	cargo build
	/usr/local/qemu/bin/qemu-system-x86_64 -drive format=raw,file=target/x86_64-stacsos_rs/debug/bootimage-stacsos-rs.bin
