use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;
fn main() -> std::io::Result<()> {
    // bindings
    let whitelist_regex = "(I|c_)?[Mm]3.*";
    let bindings = bindgen::builder()
        .header("wasm3/source/wasm3.h")
        .layout_tests(false)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .whitelist_function(whitelist_regex)
        .whitelist_type(whitelist_regex)
        .whitelist_var(whitelist_regex)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // build
    let mut cfg = cc::Build::new();
    cfg.files(
        fs::read_dir("wasm3/source")?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|p| p.extension().and_then(OsStr::to_str) == Some("c")),
    );
    if env::var_os("PROFILE") == Some(OsString::from("release"))
        && cfg.get_compiler().is_like_msvc()
    {
        cfg.flag("/Oxs");
        cfg.flag("/Oy");
        cfg.flag("/GS-");
        cfg.flag("/Zo");
        cfg.flag("/arch:AVX2");
    }
    cfg.warnings(false)
        .cpp(false)
        .extra_warnings(false)
        .include("wasm3/source")
        .compile("wasm3");
    Ok(())
}
