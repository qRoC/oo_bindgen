use foo_schema::build_lib;
use oo_bindgen::Library;
use oo_bindgen::platforms::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let lib = build_lib().expect("failed to build library schema");

    test_c_lib(&lib);
    test_dotnet_lib(&lib);
}

fn test_c_lib(lib: &Library) {
    generate_c_lib(lib);
    build_and_test_c_lib();
}

fn generate_c_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = c_oo_bindgen::CBindgenConfig {
        output_dir: PathBuf::from("tests/bindings/c/generated"),
        ffi_name: "foo_ffi".to_string(),
        platforms,
    };

    c_oo_bindgen::generate_c_package(&lib, &config).expect("failed to package C lib");
}

fn build_and_test_c_lib() {
    // Clear/create build directory
    let build_dir = PathBuf::from("tests/bindings/c/build");
    if build_dir.exists() {
        fs::remove_dir_all(&build_dir).unwrap();
    }
    fs::create_dir_all(&build_dir).unwrap();

    // CMake configure
    let result = Command::new("cmake")
        .current_dir(&build_dir)
        .arg("..")
        .status()
        .unwrap();
    assert!(result.success());

    // CMake build
    let result = Command::new("cmake")
        .current_dir(&build_dir)
        .args(&["--build", ".", ])
        .status()
        .unwrap();
    assert!(result.success());

    // Run unit tests
    let result = Command::new("ctest")
        .current_dir(&build_dir)
        .args(&[".", "-C", "Debug"])
        .status()
        .unwrap();
    assert!(result.success());
}

fn test_dotnet_lib(lib: &Library) {
    generate_dotnet_lib(lib);
    build_and_test_dotnet_lib();
}

fn generate_dotnet_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = dotnet_oo_bindgen::DotnetBindgenConfig {
        output_dir: PathBuf::from("tests/bindings/dotnet/foo"),
        ffi_name: "foo_ffi".to_string(),
        platforms,
    };

    dotnet_oo_bindgen::generate_dotnet_bindings(&lib, &config).unwrap();
}

fn build_and_test_dotnet_lib() {
    let build_dir = "tests/bindings/dotnet";
    let result = Command::new("dotnet")
        .current_dir(&build_dir)
        .arg("build")
        .status()
        .unwrap();
    assert!(result.success());

    let result = Command::new("dotnet")
        .current_dir(&build_dir)
        .arg("test")
        .status()
        .unwrap();
    assert!(result.success());
}