//! POC hook providers for `uv`.
//!
//! The [`hook_provider::HookProvider`] trait can be implemented to enable
//! external functionality to be run at various "hook points" throughout the
//! `uv` codebase.

pub mod hook_provider;
pub mod implementations;

pub use hook_provider::{HookError, HookProvider};
pub use implementations::no_op::NoOpHooksProvider;
