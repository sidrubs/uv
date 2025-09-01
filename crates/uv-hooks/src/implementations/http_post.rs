use url::Url;

use crate::{HookError, HookProvider};
use uv_distribution_types::Name;

/// A hook provider `POST`s all hooks to an HTTP server.
#[derive(Debug, Clone)]
pub struct HttpPostHookProvider {
    /// The URL to the server consuming the hook requests.
    consumption_server: Url,
}

impl HttpPostHookProvider {
    pub fn new(consumption_server: Url) -> Self {
        Self { consumption_server }
    }
}

impl HookProvider for HttpPostHookProvider {
    fn on_execute_plan(
        &self,
        plan: &uv_installer::Plan,
    ) -> impl Future<Output = Result<(), crate::HookError>> + Send {
        let Self { consumption_server } = self;
        async move {
            let serializable_plan = SerializablePlan::from(plan);
            let response = reqwest::Client::new()
                .post(
                    consumption_server
                        .join("on-execute-plan")
                        .map_err(|_e| HookError::Infrastructure {
                            message: "invalid url format".to_owned(),
                        })?
                        .as_str(),
                )
                .json(&serializable_plan)
                .send()
                .await
                .map_err(|e| HookError::Infrastructure {
                    message: e.to_string(),
                })?;

            if response.status().is_success() {
                Ok(())
            } else {
                Err(crate::HookError::ActionDenied {
                    message: response.text().await.unwrap_or_default(),
                })?
            }
        }
    }
}

use serde::Serialize;

/// A serializable version of [`uv_installer::Plan`] for HTTP transmission.
///
/// This is only a subset of the full plan for POC purposes.
#[derive(Debug, Clone, Serialize)]
pub struct SerializablePlan {
    pub cached: Vec<SerializableDistribution>,
    pub remote: Vec<SerializableDistribution>,
    pub reinstalls: Vec<SerializableDistribution>,
    pub extraneous: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SerializableDistribution {
    pub name: String,
    pub version: String,
}

impl From<&uv_installer::Plan> for SerializablePlan {
    fn from(plan: &uv_installer::Plan) -> Self {
        Self {
            cached: plan
                .cached
                .iter()
                .map(SerializableDistribution::from)
                .collect(),
            remote: plan
                .remote
                .iter()
                .map(|dist| SerializableDistribution::from(dist.as_ref()))
                .collect(),
            reinstalls: plan
                .reinstalls
                .iter()
                .map(SerializableDistribution::from)
                .collect(),
            extraneous: plan
                .extraneous
                .iter()
                .map(|dist| dist.to_string())
                .collect(),
        }
    }
}

impl From<&uv_distribution_types::InstalledDist> for SerializableDistribution {
    fn from(dist: &uv_distribution_types::InstalledDist) -> Self {
        Self {
            name: dist.name().to_string(),
            version: dist.version().to_string(),
        }
    }
}

impl From<&uv_distribution_types::Dist> for SerializableDistribution {
    fn from(dist: &uv_distribution_types::Dist) -> Self {
        Self {
            name: dist.name().to_string(),
            version: dist
                .version()
                .map_or_else(|| "unknown".to_string(), |v| v.to_string()),
        }
    }
}

impl From<&uv_distribution_types::CachedDist> for SerializableDistribution {
    fn from(dist: &uv_distribution_types::CachedDist) -> Self {
        Self {
            name: dist.name().to_string(),
            version: dist.filename().version.to_string(),
        }
    }
}
