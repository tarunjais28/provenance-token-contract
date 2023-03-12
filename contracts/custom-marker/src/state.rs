use super::*;

pub static CONFIG_KEY: &[u8] = b"config";
pub static BALANCE_KEY: &[u8] = b"balance";
pub static BLACKLIST_KEY: &[u8] = b"blacklist";
pub static SHARE_HOLDER_KEY: &[u8] = b"share_holder";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    /// The root name of the contract.
    pub contract_name: String,
    /// Country code
    pub country_codes: Vec<u8>,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Balances {
    /// balance cap for each token holder (eg. balance cap for each user = 1000,
    /// users can only hold up to 1000 tokens.
    pub bal_cap: Uint128,
    /// frozen balance
    pub frozen_bal: Uint128,
}

pub fn create_bal(storage: &mut dyn Storage) -> Bucket<Balances> {
    bucket(storage, BALANCE_KEY)
}

pub fn read_bal(storage: &mut dyn Storage) -> ReadonlyBucket<Balances> {
    bucket_read(storage, BALANCE_KEY)
}

pub fn create_blacklist(storage: &mut dyn Storage) -> Singleton<Vec<Addr>> {
    singleton(storage, BLACKLIST_KEY)
}

pub fn read_blacklist(storage: &dyn Storage) -> ReadonlySingleton<Vec<Addr>> {
    singleton_read(storage, BLACKLIST_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ShareHolder {
    pub address: Addr,
    pub amount: Uint128,
}

pub fn manage_share_holders(storage: &mut dyn Storage) -> Bucket<HashMap<Addr, Uint128>> {
    bucket(storage, SHARE_HOLDER_KEY)
}

pub fn read_share_holders(storage: &mut dyn Storage) -> ReadonlyBucket<HashMap<Addr, Uint128>> {
    bucket_read(storage, SHARE_HOLDER_KEY)
}
