// crates/windexer-network/src/consensus/mod.rs

//! Consensus module integrating with Jito staking for validator consensus

pub mod config;
pub mod protocol;
pub mod state;
pub mod validator;

pub use protocol::{ConsensusProtocol, ConsensusMessage, BlockHash};

// Remove duplicate ConsensusConfig definition since we're using the one from config.rs