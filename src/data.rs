use soroban_sdk::{contracttype, Address, Bytes, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct QuestItem {
    pub quest_id: u32,
    pub title: String,
    pub reward_token: Address,
    pub winner_count: u32,
    pub reward_rate: i128,
    pub mode: bool, //0 for raffle, 1 for fcfs
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Manager,
    SmartWalletId,
    QuestIds,
    Quest(u32),
    IsActiveQuest(u32),
    QuestCount,
    Winners(u32),
    IsWinner(u32, Address),
    ClaimStatus(u32, Address),
    GlobalPoints(Address),
}
