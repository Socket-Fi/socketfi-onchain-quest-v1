#![allow(unused)]
use crate::types::UserPoints;
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

soroban_sdk::contractimport!(
    file = "../socketfi-smart-subaccount/target/wasm32-unknown-unknown/release/socketfi_smart_subaccount.wasm"
);
