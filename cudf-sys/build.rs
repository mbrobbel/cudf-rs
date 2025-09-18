use std::env;

fn main() {
    println!("cargo:rustc-link-lib=cudf");
    println!("cargo:rustc-link-lib=rmm");
    println!("cargo:rustc-link-lib=cudart");

    cxx_build::bridge("src/lib.rs")
        .include("include")
        .include(env::var("CUDA_HOME").unwrap_or_default())
        .file("cpp/cudf_rs.cpp")
        // https://github.com/rapidsai/rmm/blob/29dd32302eb7c3e16fb837a1cfe4baac98071512/cpp/CMakeLists.txt#L115
        .flag("-DLIBCUDACXX_ENABLE_EXPERIMENTAL_MEMORY_RESOURCE")
        .flag("-std=c++20")
        .compile("cudf-sys");

    println!("cargo:rerun-if-changed=include/");
    println!("cargo:rerun-if-changed=cpp/");
}
