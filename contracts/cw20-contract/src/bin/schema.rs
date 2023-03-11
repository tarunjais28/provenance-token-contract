use cosmwasm_schema::write_api;

use token_contract::msg::{Execute, Instantiate, Query};

fn main() {
    write_api! {
        instantiate: Instantiate,
        execute: Execute,
        query: Query,
    }
}
