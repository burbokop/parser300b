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

    println!("env::current_dir(): {:?}", std::env::current_dir());


    let npm_result = Command::new("npm")
        .current_dir("tests/generator")
        .arg("install")
        .output()
        .unwrap();

    if !npm_result.status.success() {
        panic!("generator dependencies resolving failed: {}", String::from_utf8(npm_result.stdout).unwrap());
    }
    
    let tsc_result = Command::new("./node_modules/.bin/tsc")
        .current_dir("tests/generator")
        .output()
        .unwrap();

    if !tsc_result.status.success() {
        panic!("generator compilation failed: {}", String::from_utf8(tsc_result.stderr).unwrap());
    }

    println!("cargo:rerun-if-changed=tests/generator/src/main.ts");
}