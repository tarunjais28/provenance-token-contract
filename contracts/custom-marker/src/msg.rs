use super::*;

#[cw_serde]
pub struct InitMsg {
    pub name: String,
    pub country_codes: Vec<u8>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        supply: Uint128,
        denom: String,
    },
    GrantAccess {
        denom: String,
    },
    Finalize {
        denom: String,
    },
    Activate {
        denom: String,
    },
    Mint {
        amount: Uint128,
        denom: String,
    },
    Burn {
        amount: Uint128,
        denom: String,
    },
    Cancel {
        denom: String,
    },
    Destroy {
        denom: String,
    },
    Transfer {
        amount: Uint128,
        denom: String,
        to: String,
        country_code: u8,
    },
    Withdraw {
        amount: Uint128,
        denom: String,
        balances: Balances,
        country_code: u8,
    },
    Blacklist(UpdateType<Addr>),
    UpdateBalances((Addr, UpdateType<Balances>)),
}

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::Marker)]
    GetByAddress { address: String },

    #[returns(provwasm_std::Marker)]
    GetByDenom { denom: String },

    #[returns(Vec<u8>)]
    GetAuthorizedCountries {},

    #[returns(Option<Vec<ShareHolder>>)]
    GetShareHolders { denom: String },

    #[returns(Option<Vec<Addr>>)]
    GetFreezedAccounts {},

    #[returns(Balances)]
    GetBalances { address: Addr },
}
