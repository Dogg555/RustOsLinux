use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().context("failed to spawn command")?;
    if !status.success() {
        bail!("command failed with status: {status}");
    }
    Ok(())
}

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let kernel = root.join("target/i686-rustos/debug/kernel");
    let iso_dir = root.join("../iso");
    let boot_dir = iso_dir.join("boot");
    let grub_dir = boot_dir.join("grub");
    std::fs::create_dir_all(&grub_dir)?;

    run(Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("kernel")
        .arg("--target")
        .arg("i686-rustos.json")
        .current_dir(&root))?;

    std::fs::copy(&kernel, boot_dir.join("kernel.elf"))
        .with_context(|| format!("copying kernel from {}", kernel.display()))?;

    run(Command::new("grub-mkrescue")
        .arg("-o")
        .arg(root.join("target/rustos.iso"))
        .arg(&iso_dir)
        .current_dir(&root))?;

    run(Command::new("qemu-system-i386")
        .arg("-cdrom")
        .arg(root.join("target/rustos.iso"))
        .arg("-serial")
        .arg("stdio"))?;

    Ok(())
}
