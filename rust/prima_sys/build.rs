//! Generates the C bindings for the PRIMA derivative-free optimization
//! library at build time.

fn main() {
    // Skip building the bindings if we are on docs.rs, otherwise we
    // will get build failures because their images won't contain our
    // native dependencies.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo::rerun-if-changed=prima_bindgen.h");

    // We will need a Fortran compiler to build to PRIMA library.
    if std::env::var("FC").is_err() {
        panic!("The FC environment variable must point to Fortran compiler.");
    }

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Clone PRIMA from GitHub.
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.depth(1);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    let prima_clone_path = out_path.join("prima");
    builder.clone("https://github.com/libprima/prima", &prima_clone_path)
        .expect("Failed to clone PRIMA.");

    // Build PRIMA with CMake.
    let dst = cmake::Config::new(&prima_clone_path).build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=primac");

    let bindings = bindgen::Builder::default()
        .header("prima_bindgen.h")
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .generate()
        .expect("Unable to generate PRIMA C bindings.");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write PRIMA C bindings!");
}
