use namada::ledger::queries::{Client as NamadaClientTrait, EncodedResponseQuery};
use namada::types::storage::BlockHeight;
use tendermint_rpc::{Client as TendermingClient, Error, Response, SimpleRequest};

// this file contains client that is being passed to Namada
// SDK for performing the networking calls
// this is just a placeholder for now
// here is how it could look like in internet browser
// https://github.com/anoma/using-namada-sdk-poc/blob/main/usage-of-namada-sdk/src/web_namada/mod.rs

/// The clint that is being passed to all the sdk calls
/// has the implement Client and Client as per
/// C: Client + crate::ledger::queries::Client + Sync in
/// /namada/shared/src/ledger/tx.rs:submit_init_account
pub struct DesktopClient;

#[async_trait::async_trait]
impl NamadaClientTrait for DesktopClient {
    type Error = std::io::Error;

    /// this is a callback that is being called by Namada SDK to perform network calls
    async fn request(
        &self,
        _path: String,
        _data: Option<Vec<u8>>,
        _height: Option<BlockHeight>,
        _prove: bool,
    ) -> Result<EncodedResponseQuery, Self::Error> {
        // using the passed in data we perform the abci_query here
        // ... using the platform specific networking lib
        Ok(EncodedResponseQuery::default())
    }
}

/// this is needed to facilitate the network calls
#[async_trait::async_trait]
impl TendermingClient for DesktopClient {
    async fn perform<R>(&self, _request: R) -> Result<R::Response, Error>
    where
        R: SimpleRequest,
    {
        // the actual network call by tendermint-rs `Client`
        // are being done here
        Response::from_string("response".to_string())
    }
}
