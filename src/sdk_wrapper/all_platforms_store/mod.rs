use namada::ledger::wallet::Store;

// this file contains utils for wallet::Store.
// it is for persisting data in the platform specific way
// on operating system most likely a file system, in browser
// wither virtual file system which is backed by IndexedDB, LocalStorage, ...
// Or in browser it could directly call js to use those persisting facilities

pub struct DesktopStore;
impl DesktopStore {
    /// if there is an existing storage, it is being retrieved, possibly decrypted
    /// If none existed before a new one is being initialized and returned
    pub fn load_or_initialize() -> Store {
        let store = Store::default();
        store
    }
}
