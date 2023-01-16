mod sdk_wrapper;

use futures::executor::block_on;
use sdk_wrapper::account_actions::init_account_on_chain;

fn main() {
    let future = init_account_on_chain("alias_for_account".to_string());
    block_on(future);
}
