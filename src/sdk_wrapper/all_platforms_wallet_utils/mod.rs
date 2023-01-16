use namada::ledger::wallet::{Alias, ConfirmationResponse, WalletUtils};

/// platform specific implementation for the wallet utils for enabling the wallet
/// to be able to communicate with the user. Mostly for requesting a password
/// This should take JavaScript callbacks to trigger UI prompt and error
pub struct DesktopWalletUtils;
impl WalletUtils for DesktopWalletUtils {
    /// this is prefix in storage for the encoded wallet "file" or data blob in database
    type Storage = String;

    /// prompt the password or return None based on input
    fn read_and_confirm_pwd(unsafe_dont_encrypt: bool) -> Option<String> {
        if unsafe_dont_encrypt {
            return None;
        }
        // use platform specific way to request password
        // return the password. If we are on web we might like to
        // rethink this. We could likely have those values in memory before the
        // process that triggered this call even started
        let password_result = prompt_password_from_user();

        // the trait might benefit of having `Result` as a return type
        // as something can go wrong here
        let password = password_result.unwrap();
        Some(password)
    }

    /// reads a persisted password
    fn read_password(_prompt_msg: &str) -> String {
        "aaa".to_string()
    }

    /// reads a persisted alias
    fn read_alias(_prompt_msg: &str) -> String {
        "account_alias_1".to_string()
    }

    fn show_overwrite_confirmation(_alias: &Alias, _alias_for: &str) -> ConfirmationResponse {
        ConfirmationResponse::Skip
    }

    /// prompt the password or return None based on input
    fn new_password_prompt(_unsafe_dont_encrypt: bool) -> Option<String> {
        // let password = prompt_input_from_user();
        Some("password".to_string())
    }
}

#[cfg(not(feature = "target-wasm"))]
fn prompt_password_from_user() -> Result<String, std::io::Error> {
    // prompt and confirm a password from the user
    // likely any validation would also happen in this layer
    Ok("password".to_string())
}

#[cfg(not(feature = "target-wasm"))]
// wallet utils for web
// this is mostly providing the SDK any user input and data retrieval
// during the wallet processes
#[cfg(feature = "target-wasm")]
#[wasm_bindgen(raw_module = "./walletUtils")]
extern "C" {
    #[wasm_bindgen(js_class = "WalletUtils")]
    type WalletUtils;

    #[wasm_bindgen(constructor)]
    fn new() -> WalletUtils;

    #[wasm_bindgen(
        catch,
        method,
        js_class = "WalletUtils",
        js_name = "promptInputFromUser"
    )]
    async fn prompt_input_from_user(
        this: &NetworkingUtils,
        title: JsValue,
        description: bool,
    ) -> Result<JsValue, JsValue>;

    async fn prompt_password_from_user(this: &NetworkingUtils) -> Result<JsValue, JsValue>;
}
