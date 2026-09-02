//! Deterministic action primitives for Tarif.
//!
//! Specification 002 intentionally exposes only the bounded MCP `tools/call`
//! normalization surface. It is not an authorization decision engine or MCP
//! proxy.

mod action;
mod error;
mod strict_json;

pub use action::{
    ACTION_SCHEMA_V1, MCP_PROTOCOL_NAME, MCP_REVISION_2026_07_28, Action, ActionArguments,
    ArgumentState, ProtocolDescriptor, Target, canonical_bytes, normalize_and_canonicalize,
    normalize_mcp_tools_call, parse_action_ir,
};
pub use error::ActionError;
