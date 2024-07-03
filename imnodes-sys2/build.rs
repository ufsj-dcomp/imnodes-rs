extern crate bindgen;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

#[cfg(not(unix))]
compile_error!("Only unix is supported for now");

fn gen_cimnodes(cwd: &Path) -> Result<(), Box<dyn Error>> {
    if ! std::process::Command::new(cwd.join("build.sh"))
        .spawn()?
        .wait()?
        .success() {
        panic!("build script failed")
    }
    Ok(())
}

fn main() {
    let cwd = &std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cwd = Path::new(cwd);

    let cimnodes = cwd.join("cimnodes");
    let cimgui = cwd.join("cimgui");

    dbg!(&cimnodes, &cimgui);
    let header_paths = [cimnodes.join("imnodes"), cimgui.join("imgui"), cimgui];

    cc::Build::new()
        .cpp(true)
        .define("IMNODES_NAMESPACE", "imnodes")
        .define("IMGUI_API", "")
        .define("IMGUI_IMPL_API", "")
        .includes(&header_paths)
        .file(cimnodes.join("cimnodes.cpp"))
        .compile("imnodes");

    gen_cimnodes(cwd).unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(header_paths.iter().map(|p| format!("-I{}", p.display())))
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
