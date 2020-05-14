fn main() {
    println!("cargo:rerun-if-changed=src/ctest.c");
    cc::Build::new().file("src/ctest.c").compile("ctest");
}
