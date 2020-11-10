#![cfg_attr(feature = "strict", deny(warnings))]

#[cfg(feature = "client")]
pub mod client;
#[macro_use]
pub mod pack;

pub mod shared_mem {
    // TODO: import the shared_mem crate instead of hardcoding here.
    solana_sdk::declare_id!("shmem4EWT2sPdVGvTZCzXXRAURL9G5vpPxNwSeKhHUL");
}
