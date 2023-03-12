use super::*;

pub fn ensure_bal_cap_available(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom.clone())?;
    let total_supply = Uint128::from_str(&marker.total_supply.to_string())?;
    let balances = read_bal(deps.storage).load(denom.as_bytes())?;

    if (total_supply + amount) > balances.bal_cap {
        let err = "Balance capital exceeded";
        return Err(StdError::generic_err(err));
    }

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

    if amount > (bal - balances.frozen_bal) {
        let err = "Balance is frozen";
        return Err(StdError::generic_err(err));
    }

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