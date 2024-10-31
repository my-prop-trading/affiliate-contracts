use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AffiliateIntegrationSettings {
    Acqufy,
    Keitaro,
}

impl Display for AffiliateIntegrationSettings {
    // Implement the fmt method for the AffiliateIntegrationSettings enum

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Match the enum variant and return the corresponding string
        match self {
            AffiliateIntegrationSettings::Acqufy => write!(f, "acqufy"),
            AffiliateIntegrationSettings::Keitaro => write!(f, "keitaro"),
        }
    }
}