fn main() {
    // Chỉ định file assembly cần biên dịch
    cc::Build::new()
        .file("src/boot.s")
        .compile("boot");

    // Báo cho Cargo biết nếu boot.s thay đổi thì cần build lại
    println!("cargo:rerun-if-changed=src/boot.s");
}
