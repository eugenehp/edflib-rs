use std::{ env, path::PathBuf };

const EDFLIB_SOURCE_DIR: &str = "edflib-src";
const EDFLIB_HEADER: &str = "edflib.h";
const EDFLIB_SRC: &str = "edflib.c";

fn generate_bindings() {
    let edflib_header_path = PathBuf::from(EDFLIB_SOURCE_DIR).join(EDFLIB_HEADER);
    let librs_path = PathBuf::from("src").join("lib.rs");

    let bb = bindgen::Builder
        ::default()
        .derive_copy(true)
        .derive_debug(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        .derive_eq(true)
        .derive_ord(true)
        .derive_hash(true)
        .impl_debug(true)
        .merge_extern_blocks(true)
        .enable_function_attribute_detection()
        .sort_semantically(true)
        .header(edflib_header_path.to_string_lossy())
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(unused)]")
        .raw_line("pub const EDFLIBSYS_VERSION: Option<&str> = option_env!(\"CARGO_PKG_VERSION\");")
        .allowlist_file(edflib_header_path.to_string_lossy());

    let bindings = bb.generate().expect("Unable to generate bindings");
    bindings.write_to_file(librs_path).expect("Couldn't write bindings");
}

fn build() {
    let mut bb = cc::Build::new();
    let build = bb
        .files([PathBuf::from(EDFLIB_SOURCE_DIR).join(EDFLIB_SRC)])
        .include(PathBuf::from(EDFLIB_SOURCE_DIR));

    let _target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let _target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let is_release = env::var("PROFILE").unwrap() == "release";
    let _compiler = build.get_compiler();

    if is_release {
        build.define("NDEBUG", None);
    }
    build.warnings(false);
    build.compile(EDFLIB_SOURCE_DIR);
}

pub fn main() {
    println!("cargo:rerun-if-changed=./edflib-src");
    generate_bindings();
    build();
}
