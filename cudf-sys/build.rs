use std::env;

fn main() {
    println!("cargo:rustc-link-lib=cudf");
    println!("cargo:rustc-link-lib=rmm");
    println!("cargo:rustc-link-lib=cudart");

    let (bridge, cpp) = ["rmm/mod", "column", "lib", "table"]
        .into_iter()
        .map(|item| (format!("src/{item}.rs"), format!("cpp/{item}.cpp")))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let prefix = std::path::PathBuf::from(env::var("CONDA_PREFIX").unwrap_or_default());
    cxx_build::bridges(bridge)
        .include("include")
        .include(prefix.join("include").join("rapids"))
        .include(prefix.join(format!(
            "targets/{}-linux/include",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default()
        )))
        .include(prefix.join("targets/sbsa-linux/include"))
        .files(cpp)
        // https://github.com/rapidsai/rmm/blob/29dd32302eb7c3e16fb837a1cfe4baac98071512/cpp/CMakeLists.txt#L115
        .flag("-DLIBCUDACXX_ENABLE_EXPERIMENTAL_MEMORY_RESOURCE")
        .flag("-std=c++20")
        .compile("cudf-sys");

    println!("cargo:rerun-if-changed=include/");
    println!("cargo:rerun-if-changed=cpp/");
}
