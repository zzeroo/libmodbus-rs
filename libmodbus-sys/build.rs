use std::{
    env, fs, io,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let mut cfg = cc::Build::new();
    fs::create_dir_all(&include).unwrap();

    // If `pkg-config` found libmodbus we use `pkg-config` to get
    // the include_path and call bindgen with that.
    if let Ok(library) = pkg_config::probe_library("libmodbus") {
        if let Some(include) = library.include_paths.get(0) {
            run_bindgen(&include);
        }

        return;
    }

    // `pkg-config` doesn't found libmodbus.
    // So we have to compile libmodbus from source (source are in a git submodule)
    // Then run bindgen with that folder as include path set.

    if !Path::new("libmodbus/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "libmodbus"])
            .status();
    }

    // Copy over all header files
    cp_r("libmodbus/src", &include);

    let mut config_h = fs::File::create(include.join("config.h")).unwrap();
    write!(
        config_h,
        r#"
        #define VERSION "3.1.6"
    "#
    )
    .unwrap();

    let mut modbus_version_h = fs::File::create(include.join("modbus-version.h")).unwrap();
    write!(
        modbus_version_h,
        r#"
        #define LIBMODBUS_VERSION_MAJOR (3)
        #define LIBMODBUS_VERSION_MINOR (1)
        #define LIBMODBUS_VERSION_MICRO (6)
        #define LIBMODBUS_VERSION        3.1.6
        #define LIBMODBUS_VERSION_STRING "3.1.6"
        #define LIBMODBUS_VERSION_HEX ((LIBMODBUS_VERSION_MAJOR << 16) |  \
                                    (LIBMODBUS_VERSION_MINOR <<  8) |  \
                                    (LIBMODBUS_VERSION_MICRO <<  0))
        #define LIBMODBUS_VERSION_CHECK(major,minor,micro)      \
            (LIBMODBUS_VERSION_MAJOR > (major) ||               \
            (LIBMODBUS_VERSION_MAJOR == (major) &&             \
            LIBMODBUS_VERSION_MINOR > (minor)) ||             \
            (LIBMODBUS_VERSION_MAJOR == (major) &&             \
            LIBMODBUS_VERSION_MINOR == (minor) &&             \
            LIBMODBUS_VERSION_MICRO >= (micro)))
    "#
    )
    .unwrap();

    cfg.include(&include)
        .include("libmodbus/src")
        .out_dir(dst.join("build"))
        .warnings(false);

    // Include all cross-platform C files
    add_c_files(&mut cfg, "libmodbus/src");

    cfg.compile("modbus");

    println!("cargo:root={}", dst.display());

    run_bindgen(&include);
}

fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    for e in from.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let from = e.path();
        let to = to.as_ref().join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            fs::create_dir_all(&to).unwrap();
            cp_r(&from, &to);
        } else {
            println!("{} => {}", from.display(), to.display());
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn add_c_files(build: &mut cc::Build, path: impl AsRef<Path>) {
    // sort the C files to ensure a deterministic build for reproducible builds
    let dir = path.as_ref().read_dir().unwrap();
    let mut paths = dir.collect::<io::Result<Vec<_>>>().unwrap();
    paths.sort_by_key(|e| e.path());

    for e in paths {
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            // skip dirs for now
        } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}

fn run_bindgen(include: &PathBuf) {
    println!("YIPPIE bindgen");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("can't access $OUT_DIR"));
    // Configure and generate bindings.
    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include.display()))
        .bitfield_enum("modbus_error_recovery_mode")
        .blacklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("could not reate binding");

    // Write the generated bindings to an output file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
