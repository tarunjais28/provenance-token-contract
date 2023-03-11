use cosmwasm_schema::write_api;
use custom_marker::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
    }
}
