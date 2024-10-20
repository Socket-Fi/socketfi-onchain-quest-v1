use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

use crate::{
    access::{
        authenticate_admin, authenticate_manager, has_admin, read_admin, read_manager, write_admin,
        write_manager, write_smart_wallet,
    },
    data::QuestItem,
    error::ContractError,
    quest::{read_is_quest, read_quest, read_quest_count, write_is_quest, write_quest},
    reward::{send_token, take_token},
    winner::{
        add_to_winner_list, read_has_claimed, read_is_winner, read_winner_list, write_has_claimed,
        write_is_winner,
    },
};

pub trait QuestTrait {
    fn init(e: Env, admin: Address, wallet_smart_id: Address) -> Result<(), ContractError>;
    fn update_smart_wallet_id(e: Env, smart_wallet_id: Address);
    fn set_manager(e: Env, manager: Address);
    fn create_quest(
        e: Env,
        title: String,
        reward_token: Address,
        winner_count: u32,
        reward_rate: i128,
        mode: bool,
    ) -> u32;
    fn claim_reward(e: Env, quest_id: u32, winner: Address) -> Result<(), ContractError>;
    fn add_winner(e: Env, quest_id: u32, user_id: Address) -> Result<(), ContractError>;
    fn get_winner_list(e: Env, quest_id: u32) -> Result<Vec<Address>, ContractError>;
    fn selection_open(e: Env, quest_id: u32) -> bool;
    fn get_has_claimed(e: Env, quest_id: u32, winner: Address) -> Result<bool, ContractError>;

    fn get_admin(e: Env) -> Address;
    fn get_manager(e: Env) -> Address;

    fn get_quest(e: Env, quest_id: u32) -> Result<QuestItem, ContractError>;
    fn check_is_winner(e: Env, quest_id: u32, user_id: Address) -> bool;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct SocketQuest;

#[contractimpl]
impl QuestTrait for SocketQuest {
    fn init(e: Env, admin: Address, smart_wallet_id: Address) -> Result<(), ContractError> {
        let is_initialized = has_admin(&e);
        if is_initialized {
            // panic!("Has already been initiated!")
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        write_smart_wallet(&e, &smart_wallet_id);
        Ok(())
    }

    fn update_smart_wallet_id(e: Env, smart_wallet_id: Address) {
        authenticate_admin(&e);
        write_smart_wallet(&e, &smart_wallet_id);
    }

    fn set_manager(e: Env, manager: Address) {
        authenticate_admin(&e);
        write_manager(&e, &manager);
    }

    fn create_quest(
        e: Env,
        title: String,
        reward_token: Address,
        winner_count: u32,
        reward_rate: i128,
        mode: bool,
    ) -> u32 {
        authenticate_admin(&e);
        let quest_id = read_quest_count(&e) + 1;

        let new_quest = QuestItem {
            quest_id: quest_id,
            title: title,
            reward_token: reward_token.clone(),
            winner_count: winner_count,
            reward_rate: reward_rate,
            mode: mode,
        };
        let from = read_admin(&e).unwrap();
        let amount = winner_count as i128 * reward_rate;

        take_token(&e, &from, &reward_token, amount);
        write_is_quest(&e, quest_id);

        write_quest(&e, quest_id, new_quest);
        quest_id
    }

    fn add_winner(e: Env, quest_id: u32, user_id: Address) -> Result<(), ContractError> {
        authenticate_manager(&e);

        if let Some(quest) = read_quest(&e, quest_id) {
            if read_winner_list(&e, quest_id).len() < quest.winner_count {
                if !read_is_winner(&e, quest_id, user_id.clone()) {
                    write_is_winner(&e, quest_id, user_id.clone());
                    add_to_winner_list(&e, quest_id, user_id);
                    Ok(())
                } else {
                    Err(ContractError::WinnerAlreadyAdded)
                }
            } else {
                Err(ContractError::WinnersLimitReached)
            }
        } else {
            Err(ContractError::QuestNotFound)
        }
    }

    fn claim_reward(e: Env, quest_id: u32, user_id: Address) -> Result<(), ContractError> {
        if let Some(quest) = read_quest(&e, quest_id) {
            let is_winner = read_is_winner(&e, quest_id, user_id.clone());
            if is_winner {
                let has_claimed = read_has_claimed(&e, quest_id, user_id.clone());

                if !has_claimed {
                    send_token(&e, &user_id, &quest.reward_token, quest.reward_rate);
                    write_has_claimed(&e, quest_id, user_id);
                    Ok(())
                } else {
                    Err(ContractError::HasClaimedReward)
                }
            } else {
                Err(ContractError::NotWinner)
            }
        } else {
            Err(ContractError::QuestNotFound)
        }
    }

    fn get_winner_list(e: Env, quest_id: u32) -> Result<Vec<Address>, ContractError> {
        if read_is_quest(&e, quest_id) {
            Ok(read_winner_list(&e, quest_id))
        } else {
            Err(ContractError::QuestNotFound)
        }
    }

    fn selection_open(e: Env, quest_id: u32) -> bool {
        let quest = read_quest(&e, quest_id).unwrap();
        read_winner_list(&e, quest_id).len() < quest.winner_count
    }

    fn get_has_claimed(e: Env, quest_id: u32, winner: Address) -> Result<bool, ContractError> {
        let is_winner = read_is_winner(&e, quest_id, winner.clone());
        if is_winner {
            Ok(read_has_claimed(&e, quest_id, winner))
        } else {
            Err(ContractError::NotWinner)
        }
    }

    fn get_admin(e: Env) -> Address {
        let admin = read_admin(&e).unwrap();
        admin
    }

    fn get_manager(e: Env) -> Address {
        let manager = read_manager(&e).unwrap();
        manager
    }

    fn get_quest(e: Env, quest_id: u32) -> Result<QuestItem, ContractError> {
        if let Some(quest) = read_quest(&e, quest_id) {
            Ok(quest)
        } else {
            Err(ContractError::QuestNotFound) // Use a more appropriate error
        }
    }

    fn check_is_winner(e: Env, quest_id: u32, user_id: Address) -> bool {
        read_is_winner(&e, quest_id, user_id)
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        authenticate_admin(&e);
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
