extern "Rust" {
    fn __near_abi_get_greeting() -> near_sdk::__private::AbiRoot;
}
fn main() -> Result<(), std::io::Error> {
    let root_abis = vec![unsafe { __near_abi_get_greeting() }];
    let combined_root_abi = near_sdk::__private::AbiRoot::combine(root_abis);
    let contents = serde_json::to_string_pretty(&combined_root_abi)?;
    print!("{}", contents);
    Ok(())
}
