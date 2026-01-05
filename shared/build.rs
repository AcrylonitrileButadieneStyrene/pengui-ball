fn main() {
    if !std::env::var("TARGET").unwrap().starts_with("wasm") {
        return;
    }

    stylance_cli::run_silent(
        std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()),
        &stylance_cli::Config {
            output_file: Some(
                std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                    .ancestors()
                    .find(|x| x.ends_with("target"))
                    .unwrap()
                    .to_path_buf()
                    .join("bundle.css"),
            ),
            ..Default::default()
        },
        |_| (),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=src/");
}
