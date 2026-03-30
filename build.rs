fn main() {
    embuild::espidf::sysenv::output();
    println!("cargo::rustc-check-cfg=cfg(esp_idf_libc_picolibc)");
    println!("cargo::rustc-check-cfg=cfg(esp_idf_version_at_least_6_0_0)");
}
