use thiserror::Error;
use uv_installer::Plan;

/// Defines hook functions that are run at various points in the `uv` code base.
///
/// A hook provider can implement this trait to "hook" into the operation of
/// `uv` at various stages. An appropriate error can be returned to halt `uv`
/// operation after the hook.
pub trait HookProvider: Send + Sync + Clone + 'static {
    /// Indicate that the `plan` is about to be executed.
    fn on_execute_plan(&self, _plan: &Plan) -> impl Future<Output = Result<(), HookError>> + Send {
        async { Ok(()) }
    }
}

#[derive(Debug, Error)]
pub enum HookError {
    /// The current action is not allowed and the process should be aborted.
    #[error("this action is not allowed")]
    ActionDenied,
}
