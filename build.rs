fn main() {
    cc::Build::new()
        .cpp(true)
        .file("shim/shim.cpp")
        .include("shim/include")
        .flag("/std:c++17")
        .define("WIN32_LEAN_AND_MEAN", None)
        .compile("nova_shim");

    println!("cargo:rustc-link-lib=static=nova_shim");
    println!("cargo:rerun-if-changed=shim/shim.cpp");
    println!("cargo:rerun-if-changed=shim/include/nvEncodeAPI.h");
}