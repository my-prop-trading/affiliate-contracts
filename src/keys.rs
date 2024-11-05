use crate::AffiliateIntegrationSettings;

pub const GOOGLE_ANALYTICS_KEY: &str = ".google_analytics";
pub const CLICK_ID_KEY: &str = ".click_id";
pub const AFFILIATE_ID_KEY: &str = ".affiliate_id";

pub const GA_CLIENT_ID_KEY: &str = ".client_id_ga";
pub const GA_SESSION_ID_KEY: &str = ".session_id_ga";
pub const GA_CLI_ID_KEY: &str = ".gclid";

pub const UTM_SOURCE_KEY: &str = ".utm_source";
pub const UTM_MEDIUM_KEY: &str = ".utm_medium";
pub const UTM_TERM_KEY: &str = ".utm_term";

pub fn get_click_id_key(affiliate_integration_settings: AffiliateIntegrationSettings) -> String {
    let key = format!("{}_{}", CLICK_ID_KEY, affiliate_integration_settings.to_string());
    key
}

pub fn get_affiliate_id_key(affiliate_integration_settings: AffiliateIntegrationSettings) -> String {
    let key = format!("{}_{}", AFFILIATE_ID_KEY, affiliate_integration_settings.to_string());
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_click_id_key_acqufy() {
        let key = get_click_id_key(AffiliateIntegrationSettings::Acqufy);
        assert_eq!(key, ".click_id_acqufy");
    }

    #[test]
    fn test_get_click_id_key_keitaro() {
        let key = get_click_id_key(AffiliateIntegrationSettings::Keitaro);
        assert_eq!(key, ".click_id_keitaro");
    }

    #[test]
    fn test_get_affiliate_id_key_acqufy() {
        let key = get_affiliate_id_key(AffiliateIntegrationSettings::Acqufy);
        assert_eq!(key, ".affiliate_id_acqufy");
    }

    #[test]
    fn test_get_affiliate_id_key_keitaro() {
        let key = get_affiliate_id_key(AffiliateIntegrationSettings::Keitaro);
        assert_eq!(key, ".affiliate_id_keitaro");
    }
}