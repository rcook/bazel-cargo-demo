use crate::error::DataAccessError;
use crate::object_model::Widget;
use crate::result::DataAccessResult;
use anyhow::anyhow;
use tracing::instrument;

#[derive(Debug)]
pub struct WidgetRepo;

impl WidgetRepo {
    pub fn new() -> Self {
        Self
    }

    #[instrument]
    pub fn query_widgets(&self) -> DataAccessResult<Vec<Widget>> {
        Ok([
            "c3e8d6d6-c79d-4372-8925-0ec9a6795e61".parse::<Widget>(),
            "d5477aab-2d2b-4c00-8bff-1c92eb1ceb54".parse::<Widget>(),
            "8f3cffe7-0c10-4b31-acb2-40df8f48f218".parse::<Widget>(),
        ]
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|e| DataAccessError::Other(anyhow!(e)))?)
    }
}
