use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn write_empty_initramfs(path: &PathBuf) -> io::Result<()> {
    let mut image = Vec::new();
    image.extend_from_slice(b"INITRAMF");
    image.extend_from_slice(&0u32.to_le_bytes());
    image.extend_from_slice(&20u32.to_le_bytes());
    image.extend_from_slice(&20u32.to_le_bytes());
    fs::write(path, image)
}

fn main() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = manifest_dir.parent().unwrap().join("build/initramfs.img");
    let output = PathBuf::from(env::var("OUT_DIR").unwrap()).join("initramfs.img");

    println!("cargo:rerun-if-changed={}", source.display());
    if source.exists() {
        fs::copy(source, output)?;
    } else {
        write_empty_initramfs(&output)?;
    }

    Ok(())
}
