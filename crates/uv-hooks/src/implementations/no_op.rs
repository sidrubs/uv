use crate::HookProvider;

/// A hook provider that does not perform actions on any hooks.
#[derive(Debug, Clone)]
pub struct NoOpHooksProvider;

impl HookProvider for NoOpHooksProvider {}
