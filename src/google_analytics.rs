use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GoogleAnalyticsKeyValue {
    pub client_id_ga: String,
    pub session_id_ga: String,
    pub gclid: String,
}