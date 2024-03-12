#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use jsonrpsee::types::response;
use starknet::macros::short_string;
use starknet_core::{
    types::{BlockId, BlockTag, ContractErrorData, FieldElement, FunctionCall, StarknetError},
    utils::get_selector_from_name,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError};

///
/// Unit test for `starknet_call`
///
/// purpose: function request `name` to StarkGate ETH bridge contract
/// fail case: invalid block
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Hash(FieldElement::ZERO),
        )
        .await
        .err();

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Hash(FieldElement::ZERO),
        )
        .await
        .err();

    assert!(response_deoxys.is_some(), "Expected an error, but got a result");

    let is_correct_error = checking_error_format(
        response_pathfinder.as_ref().unwrap(),
        StarknetError::BlockNotFound,
    );

    assert!(is_correct_error, "Expected BlockNotFound error, but got a different error");
}


///
/// Unit test for `starknet_call`
///
/// purpose: function request `name` to StarkGate ETH bridge contract
/// fail case: invalid contract address
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::ZERO,
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::ZERO,
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

        assert!(response_deoxys.is_some(), "Expected an error, but got a result");

        let is_correct_error = checking_error_format(
            response_pathfinder.as_ref().unwrap(),
            StarknetError::ContractNotFound,
        );
    
        assert!(is_correct_error, "Expected ContractNotFound error, but got a different error");
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `name` to StarkGate ETH bridge contract
/// fail case: invalid field element
///
#[rstest]
#[tokio::test]
async fn fail_invalid_contract_entry_point_selector(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: FieldElement::ZERO,
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

    let response_pathfinder = pathfinder
    .call(
        FunctionCall {
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: FieldElement::ZERO,
            calldata: vec![],
        },
        BlockId::Tag(BlockTag::Latest),
    )
    .await
    .err();

    println!("✅ JUNO {:?}", response_deoxys);
    println!("✅ PATHFINDER {:?}", response_pathfinder);

    //checking_error(response_deoxys.unwrap(), StarknetError::ContractNotFound);
    checking_error_format(&response_deoxys.unwrap(), StarknetError::ContractNotFound);

    // assert_matches!(
    //     response_deoxys,
    //     Some(ProviderError::StarknetError(
    //         StarknetError::ContractNotFound
    //     ))
    // );
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `balanceOf` to StarkGate ETH bridge contract
/// fail case: missing call data. This is different from solely *invalid* call data, as we will see shortly
///
#[rstest]
#[tokio::test]
async fn fail_missing_contract_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

    println!("✅ JUNO {:?}", response_deoxys);
    println!("✅ PATHFINDER {:?}", response_pathfinder);

    let error_reason = ContractErrorData {
        revert_error: "ContractError".to_string(),
    };

    assert!(response_deoxys.is_some(), "Expected an error, but got a result");

        let is_correct_error = checking_error_format(
            response_pathfinder.as_ref().unwrap(),
            StarknetError::ContractError(error_reason),
        );
    
    assert!(is_correct_error, "Expected ContractError error, but got a different error");
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `balanceOf` to StarkGate ETH bridge contract
/// fail case: invalid call data. This does not cause an error upon calling the contract but returns felt 0x0
///
#[rstest]
#[tokio::test]
async fn fail_invalid_contract_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![FieldElement::ZERO],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(
        response_deoxys,
        vec![FieldElement::ZERO, FieldElement::ZERO]
    );
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `name` to StarkGate ETH bridge contract
/// fail case: too many arguments in call data
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_too_many_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![FieldElement::ZERO],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `name` to StarkGate ETH bridge contract
/// success case: should return 'Ether'
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_correct_call(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("name").unwrap(),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    let response_expected = short_string!("Ether");

    assert_eq!(response_deoxys, vec![response_expected]);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `balanceOf` to StarkGate ETH bridge contract
/// success case: must return non-zero balance
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_correct_call_with_args(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![FieldElement::from_hex_be(CONTRACT_ADDR).unwrap()],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![FieldElement::from_hex_be(CONTRACT_ADDR).unwrap()],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    let balance_deoxys = u128::try_from(response_deoxys[0]).unwrap();

    assert!(balance_deoxys > 0);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_call`
///
/// purpose: function request `sort_tokens` to JediSwap exchange, with multiple arguments.
/// success case: must return array of 2 non-zero values.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_multiple_args(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(JEDI_SWAP_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("sort_tokens").unwrap(),
                calldata: vec![
                    FieldElement::from_hex_be(STARKGATE_ETHER).unwrap(),
                    FieldElement::from_hex_be(STARKGATE_USDC).unwrap(),
                ],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .call(
            FunctionCall {
                contract_address: FieldElement::from_hex_be(JEDI_SWAP_ADDR).unwrap(),
                entry_point_selector: get_selector_from_name("sort_tokens").unwrap(),
                calldata: vec![
                    FieldElement::from_hex_be(STARKGATE_ETHER).unwrap(),
                    FieldElement::from_hex_be(STARKGATE_USDC).unwrap(),
                ],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    assert!(response_deoxys.len() == 2);
    assert_ne!(response_deoxys[0], FieldElement::ZERO);
    assert_ne!(response_deoxys[1], FieldElement::ZERO);
    assert_eq!(response_deoxys, response_pathfinder);
}
