use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Create and save state
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
        country_codes: msg.country_codes,
    })?;

    // Create a name for the contract
    let bind_name_msg = bind_name(&msg.name, env.contract.address, NameBinding::Restricted)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new()
        .add_message(bind_name_msg)
        .add_attributes(vec![
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
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<ProvenanceMsg>> {
    match msg {
        ExecuteMsg::Create {
            supply,
            denom,
            bal_cap,
            frozen_bal, country_code } => try_create(deps, supply, denom, bal_cap, frozen_bal, country_code),
        ExecuteMsg::GrantAccess { denom } => try_grant_access(denom, env.contract.address),
        ExecuteMsg::Finalize { denom } => try_finalize(denom),
        ExecuteMsg::Activate { denom } => try_activate(denom),
        ExecuteMsg::Mint {amount,denom, country_code } => try_mint(deps, amount, denom, country_code ),
        ExecuteMsg::Burn { amount, denom } => try_burn(amount, denom),
        ExecuteMsg::Cancel { denom } => try_cancel(deps, denom),
        ExecuteMsg::Destroy { denom } => try_destroy(deps, denom),
        ExecuteMsg::Withdraw { amount, denom, country_code  } => {
            try_withdraw(deps, amount, denom, env.contract.address, country_code )
        }
        ExecuteMsg::Transfer { amount, denom, to , country_code } => {
            let to = deps.api.addr_validate(&to)?;
            try_transfer(deps, amount, denom, to, env.contract.address, country_code )
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
    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    // create bal
    create_bal(deps.storage).save(
        denom.as_bytes(),
        &Balances {
            bal_cap,
            frozen_bal,
        },
    )?;

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
) -> StdResult<Response<ProvenanceMsg>> {
    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    // ensure balance is not frozen
    ensure_bal_not_frozen(deps, denom.clone(), amount)?;

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
fn try_mint(
    deps: DepsMut<ProvenanceQuery>,
    amount: Uint128,
    denom: String,
    country_code: u8,
) -> StdResult<Response<ProvenanceMsg>> {
    // ensuring country is authorized
    ensure_authorized_country(deps.storage, country_code)?;

    // ensure balance capital is not exceeded
    ensure_bal_cap_available(deps, denom.clone(), amount)?;

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
fn try_cancel(deps: DepsMut<ProvenanceQuery>, denom: String) -> StdResult<Response<ProvenanceMsg>> {
    ensure_unfrozen(deps.storage, denom.clone())?;
    let msg = cancel_marker(&denom)?;

    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.cancel")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

// Create and dispatch a message that will destroy a marker.
fn try_destroy(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
) -> StdResult<Response<ProvenanceMsg>> {
    ensure_unfrozen(deps.storage, denom.clone())?;
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

    // ensure balance is not frozen
    ensure_bal_not_frozen(deps, denom.clone(), amount)?;

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
    let config = config_read(deps.storage).load()?;
    to_binary(&config.country_codes)
}
