use std::process::Command;

fn main() {
    let lein_result = Command::new("lein")
        .current_dir("tests/mirror_parser")
        .arg("uberjar")
        .output()
        .unwrap();

    if !lein_result.status.success() {
        panic!("mirror_parser compilation failed: {}", String::from_utf8(lein_result.stderr).unwrap());
    }

    println!("cargo:rerun-if-changed=tests/mirror_parser/src/mirror_parser/core.clj");
    println!("cargo:rerun-if-changed=tests/mirror_parser/test/mirror_parser/core_test.clj");
    println!("cargo:rerun-if-changed=tests/mirror_parser/project.clj");
}