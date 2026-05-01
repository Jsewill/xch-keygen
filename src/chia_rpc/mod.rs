//! Chia RPC type definitions for daemon communication.
//!
//! This module contains the serde-serializable structs used to construct
//! and parse Chia daemon websocket messages (e.g. `add_key` commands).

pub mod daemon;
pub mod websocket;
