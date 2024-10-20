use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 299,
    WinnerAlreadyAdded = 320,
    NotAdmin = 211,
    InvalidNonce = 110,
    QuestNotFound = 230,
    NotWinner = 220,
    HasClaimedReward = 240,
    WinnersLimitReached = 410,
}
