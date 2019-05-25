use std::env;
use std::path::PathBuf;

fn find_c_files(path: &str) -> std::io::Result<Vec<String>> {
    fn recurse(path: &str, files: &mut Vec<String>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                recurse(path.to_str().unwrap(), files)?;
            } else if path.to_str().unwrap().ends_with(".c") {
                files.push(path.to_str().unwrap().to_string());
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    recurse(path, &mut files)?;
    Ok(files)
}

fn main() -> std::io::Result<()> {
    let mut ccb = cc::Build::new();
    for file in find_c_files("./deps/zstd/lib/common")? {
        ccb.file(file);
    }
    for file in find_c_files("./deps/zstd/lib/decompress")? {
        ccb.file(file);
    }
    ccb.include("./deps/zstd/lib")
        .include("./deps/zstd/lib/common")
        .include("./deps/zstd/lib/decompress")
        .compile("zstd");

    let bindings = bindgen::Builder::default()
        .header("./deps/zstd/lib/zstd.h")
        .header("./deps/zstd/lib/common/zstd_errors.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("zstd_bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
