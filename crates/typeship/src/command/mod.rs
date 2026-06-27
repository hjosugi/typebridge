//! Typed command wrappers — the part `ts-rs` and friends do *not* generate.
//!
//! Per-type renderers stop at data shapes. But a Tauri app (or any RPC client)
//! also needs the *verbs*: a typed `invoke` wrapper per command so the frontend
//! never hand-writes `invoke("workspace_snapshot")` with a stringly-typed name and
//! an untyped result. This is the same move `servant-foreign` makes in Haskell —
//! derive the client functions from the same source the types come from.
//!
//! A [`Command`] pairs a Rust command name (`snake_case`, used verbatim as the
//! `invoke` key) with a return type and ordered arguments. The generated function
//! name is the naming-iso image of the command name (`workspace_snapshot` →
//! `workspaceSnapshot`).

mod definition;
mod transport;

pub use definition::{Arg, Command};
pub use transport::Transport;
