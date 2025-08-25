use crate::HookProvider;

/// A hooks provider that does not perform actions on any hooks.
#[derive(Debug, Clone)]
pub struct NoOpHooksProvider;

impl HookProvider for NoOpHooksProvider {}
