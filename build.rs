use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=./src/styles");
    let input_css: PathBuf = Path::new("./src/styles").join("main.scss");
    let output_css: PathBuf = Path::new("./src/styles").join("output.css");
    println!("Compiling styles...");
    let css =
        grass::from_path(input_css, &grass::Options::default()).expect("failed to compile styles.");

    fs::write(output_css, css).expect("failed to write styles to file.");
    println!("Finished compiling styles...");
}
