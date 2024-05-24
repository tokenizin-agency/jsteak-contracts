use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use steak::hub::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: Empty,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
