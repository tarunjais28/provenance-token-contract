#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InitMsg, QueryMsg},
    state::{config, State},
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Storage, Uint128,
};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use provwasm_std::{
    activate_marker, bind_name, burn_marker_supply, cancel_marker, create_marker, destroy_marker,
    finalize_marker, grant_marker_access, mint_marker_supply, transfer_marker_coins,
    withdraw_coins, MarkerAccess, MarkerType, NameBinding, ProvenanceMsg, ProvenanceQuerier,
    ProvenanceQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
