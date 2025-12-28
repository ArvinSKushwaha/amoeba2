use crate::query::QueryConfig;
use crate::theme::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct AmoebaConfig {
    pub theme: Theme,
    pub query_config: QueryConfig,
}

impl Default for AmoebaConfig {
    fn default() -> Self {
        AmoebaConfig {
            theme: Theme::default(),
            query_config: QueryConfig::default(),
        }
    }
}
