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
        bal_cap: Uint128,
        frozen_bal: Uint128,
        country_code: u8,
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
        country_code: u8,
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
        country_code: u8,
    },
    Blacklist(UpdateType),
}

#[cw_serde]
pub enum UpdateType {
    Add(Addr),
    Remove(Addr),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::Marker)]
    GetByAddress { address: String },
    #[returns(provwasm_std::Marker)]
    GetByDenom { denom: String },
    #[returns(provwasm_std::Marker)]
    GetAuthorizedCountries {},
}
