//! FGP license validation module
//!
//! Handles license key validation for paid marketplace skills.
//! Supports machine fingerprinting and offline grace periods.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// Default license validation API endpoint
const DEFAULT_LICENSE_API: &str = "https://api.fgp.dev/v1/licenses/validate";

/// Offline grace period in days
const OFFLINE_GRACE_DAYS: i64 = 7;

/// License validation response from API
#[derive(Debug, Deserialize)]
pub struct LicenseValidationResponse {
    pub valid: bool,
    pub skill_id: Option<String>,
    pub skill_slug: Option<String>,
    pub download_url: Option<String>,
    pub decryption_key: Option<String>,
    pub expires_at: Option<String>,
    pub error: Option<String>,
}

/// Cached license information for offline validation
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedLicense {
    pub license_key: String,
    pub skill_slug: String,
    pub machine_fingerprint: String,
    pub validated_at: String,
    pub expires_at: Option<String>,
    pub offline_until: String,
}

/// License cache storage
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LicenseCache {
    pub licenses: Vec<CachedLicense>,
}

/// Generate a machine fingerprint for license binding
pub fn get_machine_fingerprint() -> Result<String> {
    let mut components = Vec::new();

    // Get hostname
    if let Ok(host) = hostname::get() {
        components.push(host.to_string_lossy().to_string());
    }

    // Get username
    if let Ok(user) = std::env::var("USER") {
        components.push(user);
    } else if let Ok(user) = std::env::var("USERNAME") {
        components.push(user);
    }

    // Get home directory path
    if let Some(home) = dirs::home_dir() {
        components.push(home.to_string_lossy().to_string());
    }

    // Get OS info
    components.push(std::env::consts::OS.to_string());
    components.push(std::env::consts::ARCH.to_string());

    // Hash the components
    let combined = components.join("|");
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

/// Get the license cache file path
fn license_cache_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".fgp")
        .join("licenses")
        .join("cache.json")
}

/// Load license cache
fn load_license_cache() -> Result<LicenseCache> {
    let path = license_cache_path();
    if !path.exists() {
        return Ok(LicenseCache::default());
    }
    let content = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

/// Save license cache
fn save_license_cache(cache: &LicenseCache) -> Result<()> {
    let path = license_cache_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let content = serde_json::to_string_pretty(cache)?;
    fs::write(&path, content)?;
    Ok(())
}

/// Validate a license key with the API
pub fn validate_license(
    license_key: &str,
    skill_slug: &str,
    api_url: Option<&str>,
) -> Result<LicenseValidationResponse> {
    let url = api_url.unwrap_or(DEFAULT_LICENSE_API);
    let machine_fingerprint = get_machine_fingerprint()?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "license_key": license_key,
            "skill_slug": skill_slug,
            "machine_fingerprint": machine_fingerprint
        }))
        .send()
        .context("Failed to connect to license server")?;

    if response.status().is_success() {
        let validation: LicenseValidationResponse = response.json()?;

        // Cache valid license for offline use
        if validation.valid {
            cache_license(license_key, skill_slug, &machine_fingerprint)?;
        }

        Ok(validation)
    } else if response.status().as_u16() == 404 {
        // API not found - check offline cache
        check_offline_license(license_key, skill_slug)
    } else {
        let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        bail!("License validation failed: {}", error_text)
    }
}

/// Cache a validated license for offline use
fn cache_license(license_key: &str, skill_slug: &str, machine_fingerprint: &str) -> Result<()> {
    let mut cache = load_license_cache()?;

    let now = chrono::Utc::now();
    let offline_until = now + chrono::Duration::days(OFFLINE_GRACE_DAYS);

    // Remove existing license for this skill
    cache
        .licenses
        .retain(|l| l.skill_slug != skill_slug || l.license_key != license_key);

    // Add new cached license
    cache.licenses.push(CachedLicense {
        license_key: license_key.to_string(),
        skill_slug: skill_slug.to_string(),
        machine_fingerprint: machine_fingerprint.to_string(),
        validated_at: now.to_rfc3339(),
        expires_at: None,
        offline_until: offline_until.to_rfc3339(),
    });

    save_license_cache(&cache)?;
    Ok(())
}

/// Check if we have a valid offline license
fn check_offline_license(
    license_key: &str,
    skill_slug: &str,
) -> Result<LicenseValidationResponse> {
    let cache = load_license_cache()?;
    let machine_fingerprint = get_machine_fingerprint()?;

    // Find matching cached license
    let cached = cache.licenses.iter().find(|l| {
        l.license_key == license_key
            && l.skill_slug == skill_slug
            && l.machine_fingerprint == machine_fingerprint
    });

    match cached {
        Some(license) => {
            // Check if offline grace period is still valid
            let offline_until =
                chrono::DateTime::parse_from_rfc3339(&license.offline_until)
                    .context("Invalid offline_until date")?;

            if chrono::Utc::now() < offline_until {
                Ok(LicenseValidationResponse {
                    valid: true,
                    skill_id: None,
                    skill_slug: Some(skill_slug.to_string()),
                    download_url: None,
                    decryption_key: None,
                    expires_at: Some(license.offline_until.clone()),
                    error: None,
                })
            } else {
                Ok(LicenseValidationResponse {
                    valid: false,
                    skill_id: None,
                    skill_slug: None,
                    download_url: None,
                    decryption_key: None,
                    expires_at: None,
                    error: Some("Offline grace period expired. Please connect to validate license.".to_string()),
                })
            }
        }
        None => Ok(LicenseValidationResponse {
            valid: false,
            skill_id: None,
            skill_slug: None,
            download_url: None,
            decryption_key: None,
            expires_at: None,
            error: Some("No cached license found. Please connect to validate license.".to_string()),
        }),
    }
}

/// Check if a skill requires a license (by querying the API)
pub fn check_skill_pricing(skill_slug: &str) -> Result<Option<SkillPricing>> {
    let url = format!("https://api.fgp.dev/v1/skills/{}", skill_slug);

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url).send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let skill: SkillApiResponse = resp.json()?;
            if skill.price_cents > 0 {
                Ok(Some(SkillPricing {
                    price_cents: skill.price_cents,
                    currency: skill.currency,
                    tier: skill.tier,
                }))
            } else {
                Ok(None) // Free skill
            }
        }
        Ok(_) => Ok(None), // API error or skill not found in marketplace - treat as free
        Err(_) => Ok(None), // Network error - continue with free install
    }
}

/// Skill pricing information
#[derive(Debug)]
pub struct SkillPricing {
    pub price_cents: i32,
    pub currency: String,
    pub tier: String,
}

/// API response for skill details
#[derive(Debug, Deserialize)]
struct SkillApiResponse {
    #[serde(default)]
    price_cents: i32,
    #[serde(default = "default_currency")]
    currency: String,
    #[serde(default = "default_tier")]
    tier: String,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_tier() -> String {
    "free".to_string()
}

/// Format price for display
pub fn format_price(price_cents: i32, currency: &str) -> String {
    let dollars = price_cents as f64 / 100.0;
    match currency.to_uppercase().as_str() {
        "USD" => format!("${:.2}", dollars),
        "EUR" => format!("€{:.2}", dollars),
        "GBP" => format!("£{:.2}", dollars),
        _ => format!("{:.2} {}", dollars, currency),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_fingerprint() {
        let fp1 = get_machine_fingerprint().unwrap();
        let fp2 = get_machine_fingerprint().unwrap();
        assert_eq!(fp1, fp2);
        assert_eq!(fp1.len(), 64); // SHA256 hex
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(999, "USD"), "$9.99");
        assert_eq!(format_price(2500, "USD"), "$25.00");
        assert_eq!(format_price(100, "EUR"), "€1.00");
    }
}
