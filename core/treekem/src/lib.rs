#![no_std]

#[macro_use]
extern crate sgx_tstd as std;

mod application;
mod group_state;
pub mod handshake;
mod ratchet_tree;
mod tree_math;
mod crypto;
mod test_utils;

pub use crate::application::AppKeyChain;
pub use crate::group_state::GroupState;
pub use crate::handshake::Handshake;
pub use crate::crypto::secrets::PathSecret;

// temporary
pub use crate::test_utils::init_path_secret_kvs;

#[cfg(debug_assertions)]
pub mod tests {
    use super::*;
    pub use application::tests::*;
    pub use crypto::ecies::tests::*;
}
