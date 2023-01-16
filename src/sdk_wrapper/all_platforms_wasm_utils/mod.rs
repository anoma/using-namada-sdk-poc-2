use serde_json::Value;
use std::fs::{read, read_to_string};
use std::io::Error;
use std::string::ToString;

/// Utils for dealing with the WASM files that are submitted as part of transactions
/// It's responsibilities are listing the fixed set of the WASMs, as well as loading and
/// making available to the consumer
pub struct WasmUtils;
impl WasmUtils {
    /// gives the loaded WASM binary based on the key
    /// Tries to load of from the platform specific persistance mechanism, errors
    /// errors if the retrieval fails
    pub async fn get_wasm_binary(wasm_file: WasmFile) -> Result<Vec<u8>, Error> {
        let wasm_file_name = WasmFile::get_file_name_from_checksums_file(wasm_file.get_key())?;
        let wasm_binary_result = get_wasm_file_content(wasm_file_name).await;
        wasm_binary_result
    }
}

/// represents all various WASM files that are being submitted as part of the transactions
/// In addition to the fixed set there will be need for custom user defined WASMs
pub enum WasmFile {
    VpUser,
    TxInitAccount,
}

// this will need to derive wasm-bindgen to generate glue code
impl WasmFile {
    /// turns the enum value to a string that is the key in the checksum file
    fn get_key(&self) -> String {
        match self {
            WasmFile::TxInitAccount => "tx_init_account.wasm".to_string(),
            WasmFile::VpUser => "vp_user.wasm".to_string(),
        }
    }

    /// attempting to get a specific wasm file name from the checksums json file
    /// based on the key. This is only for operating system targets.
    /// Uses env var that is defined in .cargo/config.toml
    /// Should likely fetch and cache (based on checksum) the files from a place like AWS S3
    #[cfg(not(feature = "target-wasm"))]
    fn get_file_name_from_checksums_file(key: String) -> Result<String, Error> {
        let path_to_cargo_root_directory = env!("CARGO_MANIFEST_DIR").to_string();
        let path_to_wasm_binaries_from_cargo_root =
            env!("PATH_FROM_CRATE_ROOT_TO_WASM_BINARIES").to_string();

        // generate path and get the file content
        let absolute_path_to_checksums_file = format!(
            "{path_to_cargo_root_directory}/{path_to_wasm_binaries_from_cargo_root}/checksums.json"
        );

        // read json and get the file name for requested key
        let checksums_json = read_to_string(absolute_path_to_checksums_file).unwrap();
        let checksums: Value = serde_json::from_str(checksums_json.as_str())?;

        let filename = &checksums[key];
        let filename_cleaned = filename.to_string().replace("\"", "");
        Ok(filename_cleaned)
    }
}

/// only desktop, gives the content of a wasm file
/// location is based on PATH_FROM_CRATE_ROOT_TO_WASM_BINARIES defined in /.cargo/config.toml
#[cfg(not(feature = "target-wasm"))]
async fn get_wasm_file_content(file_name: String) -> Result<Vec<u8>, std::io::Error> {
    // read the env vars
    let path_to_cargo_root_directory = env!("CARGO_MANIFEST_DIR").to_string();
    let path_to_wasm_binaries_from_cargo_root =
        env!("PATH_FROM_CRATE_ROOT_TO_WASM_BINARIES").to_string();

    // generate path and get the file content
    let absolute_path_to_wasm_binary = format!(
        "{path_to_cargo_root_directory}/{path_to_wasm_binaries_from_cargo_root}/{file_name}"
    );
    let file = read(absolute_path_to_wasm_binary);
    file
}

/// web specific functions for loading the WASM files from the browser
/// The web could also implement a caching based on the checksum file
#[cfg(feature = "target-wasm")]
#[wasm_bindgen(raw_module = "./wasmUtils")]
extern "C" {
    #[wasm_bindgen(js_class = "WasmUtils")]
    type WasmUtils;

    #[wasm_bindgen(constructor)]
    fn new() -> WasmUtils;

    #[wasm_bindgen(
        catch,
        method,
        js_class = "WasmUtils",
        js_name = "read_wasm_file_by_path"
    )]
    // in web these can be loaded from files
    async fn get_wasm_file_content(
        this: &NetworkingUtils,
        file_name: JsValue,
    ) -> Result<JsValue, JsValue>;

    /// gives the exact file name for the wasm name
    async fn get_file_name_from_checksums_file(
        this: &NetworkingUtils,
        file_short_name: JsValue,
    ) -> Result<JsValue, JsValue>;
}
