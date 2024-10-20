use soroban_sdk::{Address, Env};

use crate::data::{DataKey, QuestItem, BUMP_AMOUNT, LIFETIME_THRESHOLD};

pub fn read_quest_count(e: &Env) -> u32 {
    let key = DataKey::QuestCount;
    if let Some(count) = e.storage().instance().get(&key) {
        count
    } else {
        0
    }
}

pub fn write_quest_count(e: &Env, new_count: u32) {
    let key = DataKey::QuestCount;
    e.storage().instance().set(&key, &new_count);
}

pub fn read_quest(e: &Env, quest_id: u32) -> Option<QuestItem> {
    let key = DataKey::Quest(quest_id);

    if let Some(quest) = e.storage().persistent().get::<DataKey, QuestItem>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        Some(quest)
    } else {
        None
    }
}

pub fn write_quest(e: &Env, quest_id: u32, new_quest: QuestItem) {
    write_quest_count(e, quest_id);
    let key = DataKey::Quest(quest_id);

    e.storage().persistent().set(&key, &new_quest);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_is_quest(e: &Env, quest_id: u32) -> bool {
    let key = DataKey::IsActiveQuest(quest_id);

    e.storage()
        .instance()
        .get::<DataKey, bool>(&key)
        .unwrap_or(false)
}

pub fn write_is_quest(e: &Env, quest_id: u32) {
    let key = DataKey::IsActiveQuest(quest_id);
    e.storage().instance().set(&key, &true);
}
