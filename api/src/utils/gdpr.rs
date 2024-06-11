use std::collections::HashSet;

use actix_web::HttpRequest;
use lazy_static::lazy_static;
use regex::Regex;

const GDPR_LANGUAGES: [&str; 35] = [
    // EU and EEA countries
    "AT", // Austria
    "BE", // Belgium
    "BG", // Bulgaria
    "HR", // Croatia
    "CY", // Cyprus
    "CZ", // Czechia
    "DK", // Denmark
    "EE", // Estonia
    "FI", // Finland
    "FR", // France
    "DE", // Germany
    "GR", // Greece
    "HU", // Hungary
    "IS", // Iceland
    "IE", // Ireland
    "IT", // Italy
    "LV", // Latvia
    "LI", // Liechtenstein
    "LT", // Lithuania
    "LU", // Luxembourg
    "MT", // Malta
    "NL", // Netherlands
    "NO", // Norway
    "PL", // Poland
    "PT", // Portugal
    "RO", // Romania
    "SK", // Slovakia
    "SI", // Slovenia
    "ES", // Spain
    "SE", // Sweden
    "CH", // Switzerland
    "GB", // Great Britain
    // GDPR-Adjacent Countries
    "CN", // China
    "RU", // Russia
    "CA", // Canada

	// If more should be added, make a PR and link the related legislation and it'll be accepted asap.
];

lazy_static! {
    static ref GDPR_LANGUAGE_SET: HashSet<&'static str> = GDPR_LANGUAGES.iter().cloned().collect();
    static ref LANG_REGEX: Regex = Regex::new(r"([a-z]{2})(?:-[A-Z]{2})?(?:;q=\d+\.\d+)?").unwrap();
}

pub fn is_gdpr(req: &HttpRequest) -> bool {
    // TODO: rewrite this absurdity
    if let Some(lang_header) = req.headers().get("Accept-Language") {
        if let Ok(lang_str) = lang_header.to_str() {
            for cap in LANG_REGEX.captures_iter(lang_str) {
                if let Some(captured_string) = cap.get(1) {
                    let uppercased_string = captured_string.as_str().to_uppercase();
                    if GDPR_LANGUAGE_SET.contains(&uppercased_string[..]) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
