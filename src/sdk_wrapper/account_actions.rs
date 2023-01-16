use std::str::FromStr;

use namada::ledger::args::{SdkTypes, Tx as NamdaTx, TxInitAccount};
use namada::ledger::tx::submit_init_account;
use namada::ledger::wallet::Wallet;
use namada::types::address::Address;
use namada::types::key::common::PublicKey;

use super::all_platforms_client::DesktopClient;
use super::all_platforms_store::DesktopStore;
use super::all_platforms_wallet_utils::DesktopWalletUtils;
use super::all_platforms_wasm_utils::{WasmFile, WasmUtils};
use super::desktop_tx_utils::TxUtils;

/// init account on chain e2e
/// this has lots of side effects that are not visible here and are handled by
/// the various utils. They are:
/// - prompts user for input (for account alias and password)
/// - reads binary files (WASM files that are validity predicates)
/// - submits data to network (the transaction to the nodes)
/// - persists data (the data about the new account)
pub async fn init_account_on_chain(_source_address: String) {
    // 1. networking client
    //
    // this is the networking client
    let desktop_client = DesktopClient;

    // 2. wallet
    //
    // this is like a prefix to find the associated store from
    // where ever it is persisted (fs, localstorage, ...)
    let storage_directory = "/".to_string();
    // this is a key value storage
    let store = DesktopStore::load_or_initialize();
    // we need the wallet for the actions that we are using from the SDK
    let mut wallet = Wallet::<DesktopWalletUtils>::new(storage_directory, store);

    // 3. transaction arguments
    //
    // not working, just for satisfying the compiler for now

    // getting tx_init_account vp WASM
    let tx_init_account_data_result = WasmUtils::get_wasm_binary(WasmFile::TxInitAccount).await;
    let tx_init_account_data = tx_init_account_data_result.unwrap_or_else(|error| {
        eprintln!("Error when retrieving wasm: {}", error);
        std::process::exit(0);
    });

    // we get the wasm binary
    let vp_user_result = WasmUtils::get_wasm_binary(WasmFile::VpUser).await;
    let vp_user = vp_user_result.unwrap_or_else(|error| {
        eprintln!("Error when retrieving wasm: {}", error);
        std::process::exit(0);
    });

    // placeholder to create a dummy Tx
    let tx: NamdaTx<SdkTypes> = TxUtils::new_dummy_with_sdk_types();
    let public_key = PublicKey::from_str("public_key_hash").unwrap();
    let source_address = Address::from_str("source_address_hash").unwrap();
    let tx_init_account_args = TxInitAccount {
        tx_code_path: tx_init_account_data,
        vp_code_path: vp_user,
        tx,
        public_key,
        source: source_address,
    };

    // 4. call to the SDK
    let _init_account_future =
        submit_init_account(&desktop_client, &mut wallet, tx_init_account_args);
}
