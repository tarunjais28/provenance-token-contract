pub mod contract;
mod error;
pub mod msg;
mod state;
#[cfg(test)]
mod tests;

use crate::{
    error::ContractError,
    msg::{Execute, Instantiate, Query, UpdateType},
    state::*,
};

use cosmwasm_schema::QueryResponses;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use cw20::{
    AllowanceResponse, BalanceResponse, Cw20Coin, Expiration, Logo, MinterResponse,
    TokenInfoResponse,
};
use cw20_base::{
    allowances::{
        execute_burn_from, execute_decrease_allowance, execute_increase_allowance,
        execute_send_from, execute_transfer_from, query_allowance,
    },
    contract::{
        execute_burn, execute_mint, execute_send, execute_transfer, execute_update_marketing,
        execute_upload_logo, query_balance, query_minter, query_token_info,
    },
    state::*,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
