fn main() {
    println!("cargo:rerun-if-changed=../iso/boot/grub/grub.cfg");
    println!("cargo:rerun-if-changed=../linker/linker.ld");
}
