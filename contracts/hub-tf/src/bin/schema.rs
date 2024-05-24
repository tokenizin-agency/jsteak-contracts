use cosmwasm_schema::write_api;
use steak::{
    hub::QueryMsg,
    hub_tf::{ExecuteMsg, InstantiateMsg},
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
