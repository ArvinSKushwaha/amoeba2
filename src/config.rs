use crate::query::QueryConfig;
use crate::theme::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct AmoebaConfig {
    pub theme: Theme,
    pub query_config: QueryConfig,
}
