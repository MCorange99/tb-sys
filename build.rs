use std::path::{Path, PathBuf};

extern crate bindgen;

fn run_cmd(args: &[&'static str]){
    let mut cmd = std::process::Command::new(args[0]);
    cmd.args(&args[1..]);
    let mut child = cmd.spawn().expect("Couldnt spawn cmd");
    child.wait().expect("Couldnt wait for cmd to finish");
}

fn build_lib(cuik_path: &Path, build_dir: &Path, crate_dir: &Path) {
    std::env::set_current_dir(&cuik_path).expect("Couldnt set cwd to cuik path");
    {
        run_cmd(&["lua","build.lua", "-tb"]);
    
        let mut old = build_dir.to_path_buf();
        old.push("tb.a");

        let new = old.with_file_name("libtb.a");
        std::fs::rename(old, new).expect("Couldnt rename file from tb.a to libtb.a");
    }
    std::env::set_current_dir(&crate_dir).expect("Couldnt set cwd to crate path");
}

fn main() -> std::io::Result<()> {
    let crate_dir = std::env::current_dir().expect("Couldnt get current dir");
    let mut cuik_path = crate_dir.clone();
    cuik_path.push("Cuik");

    let mut build_dir = cuik_path.clone();
    build_dir.push("bin");

    let mut include_path = cuik_path.clone();
    include_path.push("include");

    build_lib(&cuik_path, &build_dir, &crate_dir);

    println!("cargo:rustc-link-search={}", build_dir.display());
    println!("cargo:include={}", include_path.display());
    println!("cargo:rustc-link-lib=tb");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}", include_path.display()))
        .clang_arg(&format!("-I{}", include_path.with_file_name("common").display()))
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(&format!("{}/tb.h", include_path.display()))
        .allowlist_file(&format!("{}/tb_coff.h", include_path.display()))
        .allowlist_file(&format!("{}/tb_elf.h", include_path.display()))
        .allowlist_file(&format!("{}/tb_formats.h", include_path.display()))
        .allowlist_file(&format!("{}/tb_linker.h", include_path.display()))
        .allowlist_file(&format!("{}/tb_x64.h", include_path.display()))
        .allowlist_file(&format!("{}/pool.h", include_path.with_file_name("common").display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    Ok(())
}
