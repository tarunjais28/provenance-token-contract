#![allow(dead_code)] 
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

fn get_consolidated_balance(
    deps: &DepsMut<ProvenanceQuery>,
    address: Addr,
    denom: String,
) -> StdResult<Uint128> {
    // TODO: Read balance from cosmwasm_std::BankQuery::Balance to get BalanceResponse
    let mut bal = Uint128::zero();
    if let Ok(share_holders) = read_share_holders(deps.storage).load(denom.as_bytes()) {
        for share_holder in share_holders {
            if share_holder.address.eq(&address) {
                bal += share_holder.amount;
            }
        }
    };
    Ok(bal)
}

pub fn ensure_bal_cap_available(
    deps: DepsMut<ProvenanceQuery>,
    address: Addr,
    denom: String,
) -> StdResult<Response<ProvenanceMsg>> {
    let balances = read_bal(deps.storage).load(address.as_bytes())?;
    let bal = get_consolidated_balance(&deps, address, denom)?;

    check_bal_avalaility(bal, balances.bal_cap, "Balance capital exceeded")?;

    Ok(Response::default())
}

pub fn ensure_bal_maintained(
    deps: DepsMut<ProvenanceQuery>,
    to: Addr,
    from: Addr,
    denom: String,
) -> StdResult<Response<ProvenanceMsg>> {
    let to_balances = read_bal(deps.storage)
        .load(to.as_bytes())
        .unwrap_or_default();
    let from_balances = read_bal(deps.storage)
        .load(from.as_bytes())
        .unwrap_or_default();
    let to_bal = get_consolidated_balance(&deps, to, denom.clone())?;
    let from_bal = get_consolidated_balance(&deps, from, denom)?;

    check_bal_avalaility(from_balances.frozen_bal, from_bal, "Balance is frozen")?;
    check_bal_avalaility(to_bal, to_balances.bal_cap, "Balance capital exceeded")?;

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
    manage_share_holders(storage).update(
        denom.as_bytes(),
        |share_holders: Option<Vec<ShareHolder>>| -> StdResult<_> {
            match share_holders {
                Some(mut holders) => {
                    let mut is_avail = false;
                    for holder in holders.iter_mut() {
                        if holder.address.eq(&address) {
                            holder.amount += amount;
                            is_avail = true
                        }
                    }

                    if !is_avail {
                        holders.push(ShareHolder { address, amount });
                    }

                    Ok(holders)
                }
                None => {
                    let mut holders: Vec<ShareHolder> = Vec::new();
                    holders.push(ShareHolder { address, amount });
                    Ok(holders)
                }
            }
        },
    )?;

    Ok(Response::default())
}

pub fn sub_from_share_holders(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
    amount: Uint128,
) -> StdResult<Response<ProvenanceMsg>> {
    manage_share_holders(storage).update(
        denom.as_bytes(),
        |share_holders: Option<Vec<ShareHolder>>| -> StdResult<_> {
            match share_holders {
                Some(mut holders) => {
                    let mut is_avail = false;
                    for holder in holders.iter_mut() {
                        if holder.address.eq(&address) {
                            holder.amount -= amount;
                            is_avail = true
                        }
                    }

                    if !is_avail {
                        return Err(StdError::generic_err(
                            "Share holder must have something to share.",
                        ));
                    }

                    Ok(holders)
                }
                None => Err(StdError::generic_err(
                    "Share holder must have something to share.",
                )),
            }
        },
    )?;

    adjust_share_holders(storage, denom)?;

    Ok(Response::default())
}

fn adjust_share_holders(
    storage: &mut dyn Storage,
    denom: String,
) -> StdResult<Response<ProvenanceMsg>> {
    manage_share_holders(storage).update(
        denom.as_bytes(),
        |share_holders: Option<Vec<ShareHolder>>| -> StdResult<_> {
            match share_holders {
                Some(mut holders) => {
                    holders.retain(|holder| holder.amount != Uint128::zero());
                    Ok(holders)
                }
                None => Err(StdError::generic_err("Share Holders Map can't be empty!")),
            }
        },
    )?;

    Ok(Response::default())
}
