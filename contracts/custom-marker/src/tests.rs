use super::*;
use crate::contract::*;
use cosmwasm_std::{
    coin, from_binary,
    testing::{mock_env, mock_info},
    CosmosMsg,
};
use provwasm_mocks::{mock_dependencies, must_read_binary_file};
use provwasm_std::{Marker, MarkerMsgParams, ProvenanceMsgParams};

// A helper function that will extract marker message params from a custom cosmos message.
fn unwrap_marker_params(msg: &CosmosMsg<ProvenanceMsg>) -> &MarkerMsgParams {
    match &msg {
        CosmosMsg::Custom(msg) => match &msg.params {
            ProvenanceMsgParams::Marker(mp) => mp,
            _ => panic!("unexpected provenance params"),
        },
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn valid_init() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    // Call init
    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("sender", &[]),
        InitMsg {
            name: "contract.pb".into(),
        },
    )
    .unwrap();
    // Ensure a message was created to bind the name to the contract address.
    assert_eq!(1, res.messages.len());
}

#[test]
fn create_marker() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Expected marker supply
    let expected_coin = coin(420, "budz");

    // Create marker execute message
    let msg = ExecuteMsg::Create {
        supply: Uint128::new(420),
        denom: "budz".into(),
        bal_cap: Uint128::zero(),
        frozen_bal: Uint128::zero(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::CreateMarker { coin, marker_type } => {
            assert_eq!(*coin, expected_coin);
            assert_eq!(*marker_type, MarkerType::Restricted);
        }
        _ => panic!("expected marker create params"),
    }
}

#[test]
fn grant_access() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create expected access permissions (all)
    let expected_permissions = MarkerAccess::all();

    // Create an access grant execute message
    let msg = ExecuteMsg::GrantAccess {
        denom: "budz".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::GrantMarkerAccess {
            denom,
            address,
            permissions,
        } => {
            assert_eq!(denom, "budz");
            assert_eq!(address, &env.contract.address);
            assert_eq!(*permissions, expected_permissions);
        }
        _ => panic!("expected marker grant params"),
    }
}

#[test]
fn finalize_marker() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create a finalize marker execute message
    let msg = ExecuteMsg::Finalize {
        denom: "budz".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::FinalizeMarker { denom } => {
            assert_eq!(denom, "budz");
        }
        _ => panic!("expected marker finalize params"),
    }
}

#[test]
fn activate_marker() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create an activate marker execute message
    let msg = ExecuteMsg::Activate {
        denom: "budz".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::ActivateMarker { denom } => {
            assert_eq!(denom, "budz");
        }
        _ => panic!("expected marker activate params"),
    }
}

#[test]
fn withdraw_coins() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Expected withdraw amount
    let expected_coin = coin(20, "budz");

    // Create a withdraw execute message
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(20),
        denom: "budz".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::WithdrawCoins {
            marker_denom,
            coin,
            recipient,
        } => {
            assert_eq!(marker_denom, "budz");
            assert_eq!(*coin, expected_coin);
            assert_eq!(recipient, &env.contract.address);
        }
        _ => panic!("expected marker withdraw params"),
    }
}

#[test]
fn mint_coins() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Expect to mint this amount
    let expected_coin = coin(20, "budz");

    // Create a mint coins marker handler message
    let msg = ExecuteMsg::Mint {
        amount: Uint128::new(20),
        denom: "budz".into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::MintMarkerSupply { coin } => assert_eq!(*coin, expected_coin),
        _ => panic!("expected marker mint params"),
    }
}

#[test]
fn burn_coins() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Expect to burn this amount
    let expected_coin = coin(20, "budz");

    // Create a burn coins marker handler message
    let msg = ExecuteMsg::Burn {
        amount: Uint128::new(20),
        denom: "budz".into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::BurnMarkerSupply { coin } => assert_eq!(*coin, expected_coin),
        _ => panic!("expected marker burn params"),
    }
}

#[test]
fn cancel_marker() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create a cancel marker handler message
    let msg = ExecuteMsg::Cancel {
        denom: "budz".into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::CancelMarker { denom } => assert_eq!(denom, "budz"),
        _ => panic!("expected marker cancel params"),
    }
}

#[test]
fn destroy_marker() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create a destroy marker handler message
    let msg = ExecuteMsg::Destroy {
        denom: "budz".into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::DestroyMarker { denom } => assert_eq!(denom, "budz"),
        _ => panic!("expected marker destroy params"),
    }
}

#[test]
fn transfer_coins() {
    // Create default provenance mocks.
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("sender", &[]);

    // Create a transfer execute message
    let msg = ExecuteMsg::Transfer {
        amount: Uint128::new(20),
        denom: "budz".into(),
        to: "toaddress".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    let expected_coin = coin(20, "budz");
    match unwrap_marker_params(&res.messages[0].msg) {
        MarkerMsgParams::TransferMarkerCoins { coin, to, from } => {
            assert_eq!(*coin, expected_coin);
            assert_eq!(*to, Addr::unchecked("toaddress"));
            assert_eq!(from, &env.contract.address);
        }
        _ => panic!("expected marker transfer params"),
    }
}

#[test]
fn query_marker() {
    // Create a mock querier with our expected marker.
    let bin = must_read_binary_file("testdata/marker.json");
    let expected_marker: Marker = from_binary(&bin).unwrap();
    let mut deps = mock_dependencies(&[]);
    deps.querier.with_markers(vec![expected_marker.clone()]);
    // Query and ensure we got the expected marker
    let req = QueryMsg::GetByDenom {
        denom: "nugz".into(),
    };
    let bin = query(deps.as_ref(), mock_env(), req).unwrap();
    let marker: Marker = from_binary(&bin).unwrap();
    assert_eq!(marker, expected_marker);
    assert_eq!(marker.address, "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u")
}
