fn main() {
    println!("cargo:rustc-link-lib=dylib=hello_near");
    println!(
        "cargo:rustc-link-search=all=/Users/dmitriysheleg/Documents/projects/near/rust-smart-contract-test-project/contract/target/debug"
    );
}
