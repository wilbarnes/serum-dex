use std::{io, io::Write};

use borsh::schema::{Declaration, Definition};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// Wrapper around `solana_sdk::pubkey::Pubkey` so it can implement `BorshSerialize` etc.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct Address(Pubkey);

impl From<Address> for Pubkey {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl AsRef<Pubkey> for Address {
    fn as_ref(&self) -> &Pubkey {
        &self.0
    }
}

impl AsMut<Pubkey> for Address {
    fn as_mut(&mut self) -> &mut Pubkey {
        &mut self.0
    }
}

impl From<Pubkey> for Address {
    fn from(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }
}

impl From<&Pubkey> for Address {
    fn from(pubkey: &Pubkey) -> Self {
        Self(*pubkey)
    }
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct PoolState {
    pub initialized: bool,

    pub pool_token_mint: Address,
    pub assets: Vec<AssetInfo>,

    /// Mint authority for the pool token and owner for the assets in the pool.
    pub vault_signer: Address,
    /// Nonce used to generate `vault_signer`.
    pub vault_signer_nonce: u8,

    /// Additional accounts that need to be included with every request.
    pub account_params: Vec<ParamDesc>,

    /// Meaning depends on the pool implementation.
    pub admin_key: Option<Address>,

    pub custom_state: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct AssetInfo {
    pub mint: Address,
    /// Vault should be owned by `PoolState::vault_signer`
    pub vault_address: Address,
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct ParamDesc {
    pub address: Address,
    pub writable: bool,
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum PoolRequest {
    /// Accounts:
    ///
    /// - `[]` Pool account
    /// - `[]` Pool token mint (`PoolState::pool_token_mint`)
    /// - `[]` Pool vault account for each of the N pool assets (`AssetInfo::vault_address`)
    /// - `[]` Pool vault authority (`PoolState::vault_signer`)
    /// - `[]/[writable]` Any additional accounts needed to initialize the pool
    Initialize(InitializePoolRequest),

    /// Get the creation, redemption, or swap basket.
    ///
    /// Basket is written to the retbuf account as a Vec<i64>.
    ///
    /// Accounts:
    ///
    /// - `[]` Pool account
    /// - `[]` Pool token mint (`PoolState::pool_token_mint`)
    /// - `[]` Pool vault account for each of the N pool assets (`AssetInfo::vault_address`)
    /// - `[]` Pool vault authority (`PoolState::vault_signer`)
    /// - `[writable]` retbuf account
    /// - `[]` retbuf program
    /// - `[]` Accounts in `PoolState::account_params`
    GetBasket(PoolAction),

    /// Perform a creation, redemption, or swap.
    ///
    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[writable]` Pool token mint (`PoolState::pool_token_mint`)
    /// - `[writable]` Pool vault account for each of the N pool assets (`AssetInfo::vault_address`)
    /// - `[]` Pool vault authority (`PoolState::vault_signer`)
    /// - `[writable]` User pool token account
    /// - `[writable]` User account for each of the N pool assets
    /// - `[signer]` Authority for user accounts
    /// - `[]` spl-token program
    /// - `[]/[writable]` Accounts in `PoolState::account_params`
    Transact(PoolAction),

    /// Accounts:
    ///
    /// - `[writable]` Pool account
    /// - `[writable]` Pool token mint (`PoolState::pool_token_mint`)
    /// - `[writable]` Pool vault account for each of the N pool assets (`AssetInfo::vault_address`)
    /// - `[]` Pool vault authority (`PoolState::vault_signer`)
    /// - `[]/[writable]` Accounts in `PoolState::account_params`
    /// - `[]/[writable]` Custom accounts
    AdminRequest,

    CustomRequest(Vec<u8>),
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct InitializePoolRequest {
    pub vault_signer_nonce: u8,
    pub assets_length: u8,
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum PoolAction {
    /// Create pool tokens by depositing assets into the pool.
    Create(u64),
    /// Redeem pool tokens by burning the token and receiving assets from the pool.
    Redeem(u64),
    /// Deposit assets into the pool and receive other assets from the pool.
    Swap(Vec<u64>),
}

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct Basket {
    /// Must have the same length as `PoolState::assets`. Each item corresponds to
    /// one of the assets in `PoolState::assets`.
    pub quantities: Vec<i64>,
}

impl BorshSerialize for Address {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        BorshSerialize::serialize(&self.0.to_bytes(), writer)
    }
}

impl BorshDeserialize for Address {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        Ok(Self(Pubkey::new_from_array(BorshDeserialize::deserialize(
            buf,
        )?)))
    }
}

impl BorshSchema for Address {
    fn add_definitions_recursively(definitions: &mut HashMap<Declaration, Definition>) {
        Self::add_definition(
            Self::declaration(),
            Definition::Struct {
                fields: borsh::schema::Fields::UnnamedFields(vec![
                    <[u8; 32] as BorshSchema>::declaration(),
                ]),
            },
            definitions,
        );
        <[u8; 32] as BorshSchema>::add_definitions_recursively(definitions);
    }

    fn declaration() -> Declaration {
        "Address".to_string()
    }
}
