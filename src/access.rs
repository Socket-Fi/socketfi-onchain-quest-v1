use soroban_sdk::{Address, Env};

use crate::data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn authenticate_admin(e: &Env) {
    let admin = read_admin(e).unwrap();
    admin.require_auth();
}

pub fn read_manager(e: &Env) -> Option<Address> {
    let key = DataKey::Manager;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_manager(e: &Env, manager: &Address) {
    let key = DataKey::Manager;
    e.storage().instance().set(&key, manager);
}

pub fn authenticate_manager(e: &Env) {
    let manager = read_manager(e).unwrap();
    manager.require_auth();
}

pub fn read_smart_wallet(e: &Env) -> Option<Address> {
    let key = DataKey::SmartWalletId;
    e.storage()
        .instance()
        .get(&key)
        .expect("Controller not found!")
}

pub fn write_smart_wallet(e: &Env, smart_wallet_id: &Address) {
    let key = DataKey::SmartWalletId;
    e.storage().instance().set(&key, smart_wallet_id);
}
