use crate::accounts::entity::PoolPrices;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use serum_common::pack::*;
use solana_client_gen::solana_sdk::pubkey::Pubkey;

/// A Generation account stores the staking pool price for a given node Entity
/// as of a given generation--marked to the last invocation of
/// `StartStakeWithdrawal` *before* a node entity goes inactive.
///
/// This is used to determine the payout reward for node members withdrawing
/// their stake *after* their node transitions to the inactive stake, since they
/// should not receive subsequent rewards from the staking pool.
///
/// Marking the price this ways relies on the fact that the price of
/// a staking pool token can only go up (since the underlying basket can't
/// be removed or destroyed without redeeming a staking pool token).
///
/// Note that the *first* Member associated with an Entity to withdraw their
/// stake must pay for the initialization of the Generation account.
#[derive(Default, Debug, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct Generation {
    pub initialized: bool,
    pub entity: Pubkey,
    pub generation: u64,
    pub last_active_prices: PoolPrices,
}

serum_common::packable!(Generation);
