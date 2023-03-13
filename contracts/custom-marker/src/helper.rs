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

fn get_consolidated_balance(deps: &DepsMut<ProvenanceQuery>, address: Addr) -> StdResult<Uint128> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_address(address)?;
    Ok(marker.coins.iter().map(|coin| coin.amount).sum())
}

pub fn ensure_bal_cap_available(
    deps: DepsMut<ProvenanceQuery>,
    address: Addr,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    let balances = read_bal(deps.storage).load(address.as_bytes())?;
    let bal = get_consolidated_balance(&deps, address)?;

    check_bal_avalaility(bal + amount, balances.bal_cap, "Balance capital exceeded")?;

    Ok(Response::default())
}

pub fn ensure_bal_maintained(
    deps: DepsMut<ProvenanceQuery>,
    to: Addr,
    from: Addr,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    let from_balances = read_bal(deps.storage).load(to.as_bytes())?;
    let balances = read_bal(deps.storage).load(from.as_bytes())?;
    let from_bal = get_consolidated_balance(&deps, to)?;
    let bal = get_consolidated_balance(&deps, from)?;

    check_bal_avalaility(
        amount,
        from_bal - from_balances.frozen_bal,
        "Balance is frozen",
    )?;
    check_bal_avalaility(bal + amount, balances.bal_cap, "Balance capital exceeded")?;

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
