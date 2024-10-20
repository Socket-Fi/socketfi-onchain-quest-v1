use soroban_sdk::{Address, Env, String, Vec};

use crate::data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};

pub fn read_is_winner(e: &Env, quest_id: u32, user_id: Address) -> bool {
    let key = DataKey::IsWinner(quest_id, user_id);
    e.storage()
        .instance()
        .get::<DataKey, bool>(&key)
        .unwrap_or(false)
}

pub fn write_is_winner(e: &Env, quest_id: u32, user_id: Address) {
    let key = DataKey::IsWinner(quest_id, user_id);
    e.storage().instance().set(&key, &true);
}

pub fn read_has_claimed(e: &Env, quest_id: u32, winner: Address) -> bool {
    let key = DataKey::ClaimStatus(quest_id, winner);
    e.storage()
        .instance()
        .get::<DataKey, bool>(&key)
        .unwrap_or(false)
}

pub fn write_has_claimed(e: &Env, quest_id: u32, user_id: Address) {
    let key = DataKey::ClaimStatus(quest_id, user_id);
    e.storage().instance().set(&key, &true);
}

pub fn read_winner_list(e: &Env, quest_id: u32) -> Vec<Address> {
    let key = DataKey::Winners(quest_id);
    let default_list: Vec<Address> = Vec::new(&e);

    e.storage()
        .instance()
        .get::<DataKey, Vec<Address>>(&key)
        .unwrap_or(default_list)
}

pub fn write_winner_list(e: &Env, quest_id: u32, winner_list: Vec<Address>) {
    let key = DataKey::Winners(quest_id);
    e.storage().instance().set(&key, &winner_list);
}

pub fn add_to_winner_list(e: &Env, quest_id: u32, add_user: Address) {
    let mut winner_list = read_winner_list(e, quest_id);
    winner_list.push_back(add_user);
    write_winner_list(e, quest_id, winner_list);
}
