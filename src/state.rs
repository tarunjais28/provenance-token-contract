use super::*;

pub const FROZEN_BALANCES: Map<&Addr, Uint128> = Map::new("frozen_balances");
pub const BALANCE_CAP: Item<Uint128> = Item::new("balance_cap");
