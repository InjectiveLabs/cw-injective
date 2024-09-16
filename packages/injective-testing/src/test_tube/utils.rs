use injective_test_tube::{InjectiveTestApp, SigningAccount, Wasm};

pub fn wasm_file(contract_name: String) -> String {
    let snaked_name = contract_name.replace('-', "_");
    let arch = std::env::consts::ARCH;

    let target = format!("../../target/wasm32-unknown-unknown/release/{snaked_name}.wasm");

    let artifacts_dir = std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_string());
    let arch_target = format!("../../{artifacts_dir}/{snaked_name}-{arch}.wasm");

    if std::path::Path::new(&target).exists() {
        target
    } else if std::path::Path::new(&arch_target).exists() {
        arch_target
    } else {
        format!("../../{artifacts_dir}/{snaked_name}.wasm")
    }
}

pub fn store_code(wasm: &Wasm<InjectiveTestApp>, owner: &SigningAccount, contract_name: String) -> u64 {
    let wasm_byte_code = std::fs::read(wasm_file(contract_name)).unwrap();
    wasm.store_code(&wasm_byte_code, None, owner).unwrap().data.code_id
}
