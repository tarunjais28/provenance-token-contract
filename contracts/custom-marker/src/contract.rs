use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Create and save state
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
        country_codes: msg.country_codes,
    })?;

    create_blacklist(deps.storage).save(&Vec::new())?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attributes(vec![
        attr("action", "provwasm.contracts.custom_marker.init"),
        attr("integration_test", "v2"),
        attr("contract_name", msg.name),
    ]))
}

/// Handle messages that create and interact with with native provenance markers.
#[entry_point]
pub fn execute(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ProvenanceMsg>> {
    match msg {
        ExecuteMsg::Create {
            supply,
            denom,
            bal_cap,
            frozen_bal,
            country_code,
        } => try_create(deps, supply, denom, bal_cap, frozen_bal, country_code),
        ExecuteMsg::GrantAccess { denom } => try_grant_access(denom, env.contract.address),
        ExecuteMsg::Finalize { denom } => try_finalize(denom),
        ExecuteMsg::Activate { denom } => try_activate(denom),
        ExecuteMsg::Mint { amount, denom } => try_mint(amount, denom),
        ExecuteMsg::Burn { amount, denom } => try_burn(amount, denom),
        ExecuteMsg::Cancel { denom } => try_cancel(denom),
        ExecuteMsg::Destroy { denom } => try_destroy(denom),
        ExecuteMsg::Withdraw {
            amount,
            denom,
            country_code,
            balances,
        } => try_withdraw(deps, amount, denom, info.sender, country_code, balances),
        ExecuteMsg::Transfer {
            amount,
            denom,
            to,
            country_code,
        } => {
            let to = deps.api.addr_validate(&to)?;
            try_transfer(deps, amount, denom, to, info.sender, country_code)
        }
        ExecuteMsg::Blacklist(update_type) => try_update_blacklist(deps, info, update_type),
        ExecuteMsg::UpdateBalances((address, update_type)) => {
            try_update_balances(deps, update_type, address)
        }
    }
}

// Create and dispatch a message that will create a new restricted marker w/ proposed status.
fn try_create(
    deps: DepsMut<ProvenanceQuery>,
    supply: Uint128,
    denom: String,
    bal_cap: Uint128,
    frozen_bal: Uint128,
    country_code: u8,
) -> StdResult<Response<ProvenanceMsg>> {
    // TODO: Need to add management of bal_cap, frozen_bal and country_code

    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    let msg = create_marker(supply.u128(), &denom, MarkerType::Restricted)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.create")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_supply", supply)
        .add_attribute("marker_denom", denom)
        .add_attribute("balance_capital", bal_cap)
        .add_attribute("frozen_balance", frozen_bal);

    Ok(res)
}

// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_access(denom: String, address: Addr) -> StdResult<Response<ProvenanceMsg>> {
    let msg = grant_marker_access(&denom, address.clone(), MarkerAccess::all())?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.grant_access")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom)
        .add_attribute("marker_addr", address);

    Ok(res)
}

// Create and dispatch a message that will finalize a proposed marker.
fn try_finalize(denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = finalize_marker(&denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.finalize")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will activate a finalized marker.
fn try_activate(denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = activate_marker(&denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.activate")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will withdraw coins from a marker.
fn try_withdraw(
    deps: DepsMut<ProvenanceQuery>,
    amount: Uint128,
    denom: String,
    recipient: Addr,
    country_code: u8,
    balances: Balances,
) -> StdResult<Response<ProvenanceMsg>> {
    // store balances
    create_bal(deps.storage).save(recipient.as_bytes(), &balances)?;

    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    // ensure not blacklisted
    ensure_not_blacklisted(deps.storage, vec![recipient.clone()])?;

    // update share holders
    add_share_holders(deps.storage, denom.clone(), recipient.clone(), amount)?;

    // ensure balance capital maintained
    ensure_bal_cap_available(deps, recipient.clone(), amount)?;

    let marker_denom = denom.clone();
    let msg = withdraw_coins(&marker_denom, amount.u128(), &denom, recipient.clone())?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.withdraw")
        .add_attribute("integration_test", "v2")
        .add_attribute("withdraw_amount", amount)
        .add_attribute("withdraw_denom", denom)
        .add_attribute("withdraw_recipient", recipient);

    Ok(res)
}

// Create and dispatch a message that will mint coins into a marker.
fn try_mint(amount: Uint128, denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = mint_marker_supply(amount.u128(), &denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.mint")
        .add_attribute("integration_test", "v2")
        .add_attribute("mint_amount", amount)
        .add_attribute("mint_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will burn coins from a marker.
fn try_burn(amount: Uint128, denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = burn_marker_supply(amount.u128(), &denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.burn")
        .add_attribute("integration_test", "v2")
        .add_attribute("mint_amount", amount)
        .add_attribute("mint_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will cancel a marker.
fn try_cancel(denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = cancel_marker(&denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.cancel")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will destroy a marker.
fn try_destroy(denom: String) -> StdResult<Response<ProvenanceMsg>> {
    let msg = destroy_marker(denom.clone())?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.destroy")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will transfer coins from one account to another.
fn try_transfer(
    deps: DepsMut<ProvenanceQuery>,
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
    country_code: u8,
) -> StdResult<Response<ProvenanceMsg>> {
    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    // ensure not blacklisted
    ensure_not_blacklisted(deps.storage, vec![to.clone(), from.clone()])?;

    // update share holders
    add_share_holders(deps.storage, denom.clone(), to.clone(), amount)?;
    sub_from_share_holders(deps.storage, denom.clone(), from.clone(), amount)?;

    // ensure balance maintained
    ensure_bal_maintained(deps, to.clone(), from.clone(), amount)?;

    let msg = transfer_marker_coins(amount.u128(), &denom, to.clone(), from.clone())?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.transfer")
        .add_attribute("integration_test", "v2")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);

    Ok(res)
}

// Update blacklist.
fn try_update_blacklist(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    update_type: UpdateType<Addr>,
) -> StdResult<Response<ProvenanceMsg>> {
    // ensure not blacklisted
    ensure_not_blacklisted(deps.storage, vec![info.sender])?;

    // TODO: Need to add proper admin management

    match update_type.clone() {
        UpdateType::Add(addr) => {
            create_blacklist(deps.storage).update(|mut addresses: Vec<Addr>| -> StdResult<_> {
                Ok({
                    addresses.push(addr);
                    addresses
                })
            })?;
        }
        UpdateType::Remove(addr) => {
            create_blacklist(deps.storage).update(|mut addresses: Vec<Addr>| -> StdResult<_> {
                Ok({
                    addresses.retain(|address| address != &addr);
                    addresses
                })
            })?;
        }
    }

    let res = Response::new()
        .add_attribute("action", "update_blacklist")
        .add_attribute("update_type", format!("{:?}", &update_type));

    Ok(res)
}

// Update frozen balance.
fn try_update_balances(
    deps: DepsMut<ProvenanceQuery>,
    update_type: UpdateType<Balances>,
    address: Addr,
) -> StdResult<Response<ProvenanceMsg>> {
    // TODO: Need to add proper admin management

    match update_type.clone() {
        UpdateType::Add(bal) => {
            if let Ok(mut balances) = create_bal(deps.storage).load(address.as_bytes()) {
                balances.add(bal);
            } else {
                create_bal(deps.storage).save(address.as_bytes(), &bal)?;
            }
        }
        UpdateType::Remove(bal) => {
            let mut balances = create_bal(deps.storage).load(address.as_bytes())?;
            balances.sub(bal);
        }
    }

    let res = Response::new()
        .add_attribute("action", "update_balances")
        .add_attribute("address", format!("{:?}", &address))
        .add_attribute("update_type", format!("{:?}", &update_type));

    Ok(res)
}

/// Handle query requests for the provenance marker module.
#[entry_point]
pub fn query(
    deps: Deps<ProvenanceQuery>,
    _env: Env,
    msg: QueryMsg,
) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetByAddress { address } => try_get_marker_by_address(deps, address),
        QueryMsg::GetByDenom { denom } => try_get_marker_by_denom(deps, denom),
        QueryMsg::GetAuthorizedCountries {} => try_get_auth_countries(deps),
        QueryMsg::GetShareHolders { denom } => try_get_share_holders_by_denom(deps, denom),
        QueryMsg::GetFreezedAccounts {} => try_get_freezed_accounts(deps),
    }
}

// Query a marker by address.
fn try_get_marker_by_address(
    deps: Deps<ProvenanceQuery>,
    address: String,
) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_address(address)?;
    to_binary(&marker)
}

// Query a marker by denom.
fn try_get_marker_by_denom(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom)?;
    to_binary(&marker)
}

// Query authorized countries.
fn try_get_auth_countries(deps: Deps<ProvenanceQuery>) -> Result<QueryResponse, StdError> {
    let state = config_read(deps.storage).load()?;
    to_binary(&state.country_codes)
}

// Query share holders by denom.
fn try_get_share_holders_by_denom(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let share_holders = read_share_holders(deps.storage).load(denom.as_bytes()).ok();
    to_binary(&share_holders)
}

// Query freezed accounts.
fn try_get_freezed_accounts(deps: Deps<ProvenanceQuery>) -> Result<QueryResponse, StdError> {
    let accounts = read_blacklist(deps.storage).load().ok();
    to_binary(&accounts)
}
