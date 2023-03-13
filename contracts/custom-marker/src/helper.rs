use super::*;

pub fn check_bal_avalaility(
    amount: Uint128,
    capital: Uint128,
    err: &str,
) -> StdResult<Response<ProvenanceMsg>> {
    if amount > capital {
        return Err(StdError::generic_err(err));
    }

    Ok(Response::default())
}

pub fn ensure_bal_cap_available(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom.clone())?;
    let total_supply = Uint128::from_str(&marker.total_supply.to_string())?;
    let balances = read_bal(deps.storage).load(denom.as_bytes())?;

    check_bal_avalaility(
        total_supply + amount,
        balances.bal_cap,
        "Balance capital exceeded",
    )?;

    Ok(Response::default())
}

pub fn ensure_bal_not_frozen(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom.clone())?;
    let bal: Uint128 = marker.coins.iter().map(|coin| coin.amount).sum();
    let balances = read_bal(deps.storage).load(denom.as_bytes())?;

    check_bal_avalaility(amount, bal - balances.frozen_bal, "Balance is frozen")?;

    Ok(Response::default())
}

pub fn ensure_unfrozen(
    storage: &mut dyn Storage,
    denom: String,
) -> StdResult<Response<ProvenanceMsg>> {
    let balances = read_bal(storage).load(denom.as_bytes())?;

    if !balances.frozen_bal.is_zero() {
        let err = "Balance is frozen";
        return Err(StdError::generic_err(err));
    }

    Ok(Response::default())
}

pub fn ensure_authorized_country(
    storage: &mut dyn Storage,
    country_code: u8,
) -> StdResult<Response<ProvenanceMsg>> {
    let config = config_read(storage).load()?;

    if !config.country_codes.contains(&country_code) {
        let err = "Unauthorized Country";
        return Err(StdError::generic_err(err));
    }

    Ok(Response::default())
}

pub fn ensure_not_blacklisted(
    storage: &mut dyn Storage,
    address: Vec<Addr>,
) -> StdResult<Response<ProvenanceMsg>> {
    let blacklist = read_blacklist(storage).load()?;

    for addr in address {
        if blacklist.contains(&addr) {
            let err = "Unauthorized: Account Blacklisted";
            return Err(StdError::generic_err(err));
        }
    }

    Ok(Response::default())
}

pub fn add_share_holders(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    if let Ok(mut share_holders) = manage_share_holders(storage).load(&denom.as_bytes()) {
        share_holders
            .entry(address)
            .and_modify(|amt| *amt += amount)
            .or_insert(amount);
    }

    Ok(Response::default())
}

pub fn sub_from_share_holders(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    if let Ok(mut share_holders) = manage_share_holders(storage).load(&denom.as_bytes()) {
        share_holders
            .entry(address.clone())
            .and_modify(|amt| *amt -= amount);
    }

    adjust_share_holders(storage, denom, address)?;

    Ok(Response::default())
}

fn adjust_share_holders(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
) -> StdResult<Response<ProvenanceMsg>> {
    if let Ok(mut share_holders) = manage_share_holders(storage).load(&denom.as_bytes()) {
        if let Some(amount) = share_holders.get(&address) {
            if amount.is_zero() {
                share_holders.remove(&address);
            }
        }
    }

    Ok(Response::default())
}
