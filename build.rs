// The code for this file can be found at the following URL:
// https://github.com/rust-osdev/bootloader/blob/main/docs/create-disk-image.md
use std::path::PathBuf;

fn main()
{
    println!("cargo::rerun-if-changed=src/main.rs");
    
    // The output directory that build scripts should use
    let output_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    
    // Set by cargo's artifact dependency feature
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL").unwrap());
    
    // Create a UEFI disk image
    let uefi_path = output_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel).create_disk_image(&uefi_path).unwrap();
    
    // Create a BIOS disk image
    let bios_path = output_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel).create_disk_image(&bios_path).unwrap();
    
    // Pass the disk image paths as environment variables to 'src/main.rs'
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}