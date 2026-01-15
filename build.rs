fn main() {
    // Bảo Rust build file assembly
    println!("cargo:rerun-if-changed=src/boot.s");
    cc::Build::new()
        .asm_arch("armv7-a")
        .file("src/boot.s")
        .compile("boot");
}
