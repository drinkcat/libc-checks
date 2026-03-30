fn main() {
    embuild::espidf::sysenv::output();
    println!("cargo::rustc-check-cfg=cfg(esp_idf_libc_picolibc)");
}
