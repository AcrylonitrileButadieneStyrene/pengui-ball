fn main() {
    if !std::env::var("TARGET").unwrap().starts_with("wasm") {
        return;
    }

    stylance_cli::run_silent(
        &stylance_cli::Config::load(std::path::PathBuf::from(
            &std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        ))
        .unwrap(),
        |_| (),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=src/");
}
