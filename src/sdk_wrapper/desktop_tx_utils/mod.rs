use namada::ledger::args::SdkTypes;
use namada::ledger::args::Tx as NamadaTx;
use namada::types::address::EstablishedAddressGen;
use namada::types::token::Amount;
use namada::types::transaction::GasLimit;

/// Tx util that can create args::Tx, when running in browser
/// this would likely get string that came from the UI and create a Tx from them
/// This could also get some configuration vars from the JavaScript application
/// So this should likely have been passed some callbacks tat could request data from the browser
/// those are user input, application configuration and random value generator
pub struct TxUtils;
impl TxUtils {
    /// creates a Tx<SdkTypes> based on user input and application configurations.
    pub fn new_dummy_with_sdk_types() -> NamadaTx<SdkTypes> {
        let mut fee_token_address_generator = EstablishedAddressGen::new("seed");
        let fee_token_address =
            &fee_token_address_generator.generate_address(vec![0, 0, 0, 0, 0, 0, 0, 0]);
        let gas_limit = GasLimit::from(1);
        let tx_code_path: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let tx_to_return = NamadaTx {
            dry_run: false,
            force: false,
            broadcast_only: false,
            ledger_address: (),
            initialized_account_alias: Some("initialized_account_alias".to_string()),
            fee_amount: Amount::from(1),
            fee_token: fee_token_address.to_owned(),
            gas_limit: gas_limit,
            signing_key: None,
            signer: None,
            tx_code_path: tx_code_path,
        };
        tx_to_return
    }
}
