use crate::Hooks;

/// A hooks provider that does not perform actions on any hooks.
pub struct NoOpHooksProvider;

impl Hooks for NoOpHooksProvider {}
