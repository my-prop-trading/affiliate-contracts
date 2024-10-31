use crate::AffiliateIntegrationSettings;

pub const GOOGLE_ANALYTICS_KEY: &str = "google_analytics";

pub const CLICK_ID_KEY: &str = "click_id";

pub fn get_click_id_key(affiliate_integration_settings: AffiliateIntegrationSettings) -> String {
    let key = format!("{}_{}", CLICK_ID_KEY, affiliate_integration_settings.to_string());
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_click_id_key_acqufy() {
        let key = get_click_id_key(AffiliateIntegrationSettings::Acqufy);
        assert_eq!(key, "click_id_acqufy");
    }

    #[test]
    fn test_get_click_id_key_keitaro() {
        let key = get_click_id_key(AffiliateIntegrationSettings::Keitaro);
        assert_eq!(key, "click_id_keitaro");
    }
}