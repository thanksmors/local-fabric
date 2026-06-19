//! ACP (Agent Client Protocol) integration: the architectural heart of the
//! harness. One protocol, one connection, thin per-agent launchers.
//!
//! * [`protocol`] — pure JSON-RPC + ACP message types and event normalization.
//! * [`connection`] — the async JSON-RPC peer over agent stdio.
//! * [`launchers`] — how to start each agent as an ACP subprocess.

pub mod connection;
pub mod launchers;
pub mod protocol;

pub use connection::{AcpConnection, UiEvent};
pub use launchers::AgentKind;
