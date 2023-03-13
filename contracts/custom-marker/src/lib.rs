#![warn(clippy::all)]
pub mod contract;
mod error;
mod helper;
pub mod msg;
pub mod state;

#[cfg(test)]
mod tests;

use crate::{error::ContractError, helper::*, msg::*, state::*};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Storage, Uint128,
};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use provwasm_std::{
    activate_marker, burn_marker_supply, cancel_marker, create_marker, destroy_marker,
    finalize_marker, grant_marker_access, mint_marker_supply, transfer_marker_coins,
    withdraw_coins, MarkerAccess, MarkerType, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
