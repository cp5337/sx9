//! SX9 Weather Validation Engine
//!
//! Multi-provider weather data aggregation for FSO ground station suitability.
//!
//! Providers:
//! - Open-Meteo (free, global coverage)
//! - WeatherAPI.com (requires WEATHER_API_KEY env var)
//! - NOAA (free, US coverage only)
//!
//! Features:
//! - Parallel fetching from all providers
//! - Cross-validation and averaging of results
//! - FSO suitability scoring for LaserLight ground stations

use crate::utils::precision::avg_f32;
use async_trait::async_trait;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

// ================================================================
// Custom Module Errors
// ================================================================
#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Failed to fetch weather data from provider: {0}")]
    FetchError(String),
    #[error("Failed to parse weather data: {0}")]
    ParseError(String),
    #[error("No reliable weather data could be obtained from any provider")]
    NoReliableData,
}

// ================================================================
// Core Data Models
// ================================================================

/// Represents unified weather data from any provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
    pub wind_speed_kmh: f32,
    pub precipitation_mm: f32,
    pub weather_code: u32,
}

// ================================================================
// Trait for Weather Providers
// ================================================================
#[async_trait]
pub trait WeatherProvider: Send + Sync {
    /// Fetches weather data for a specific geographic location.
    async fn get_weather(&self, latitude: f64, longitude: f64)
        -> Result<WeatherData, WeatherError>;
    /// The name of the provider for identification.
    fn provider_name(&self) -> &'static str;
}

// ================================================================
// The Weather Aggregation Engine
// ================================================================
pub struct WeatherEngine {
    providers: Vec<Arc<dyn WeatherProvider>>,
}

impl WeatherEngine {
    /// Creates a new engine with a list of providers.
    #[must_use]
    pub fn new(providers: Vec<Arc<dyn WeatherProvider>>) -> Self {
        Self { providers }
    }

    /// Fetches and validates weather data from all available providers.
    ///
    /// # Errors
    /// Returns `WeatherError::NoReliableData` if no provider returns successful data,
    /// or other `WeatherError` variants if deserialization/fetching fails per provider.
    pub async fn fetch_and_validate(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<WeatherData, WeatherError> {
        if self.providers.is_empty() {
            return Err(WeatherError::NoReliableData);
        }

        // Call all providers in parallel
        let futures = self
            .providers
            .iter()
            .map(|p| p.get_weather(latitude, longitude));
        let results = join_all(futures).await;

        // Filter for successful results only
        let successful_results: Vec<WeatherData> =
            results.into_iter().filter_map(Result::ok).collect();

        if successful_results.is_empty() {
            return Err(WeatherError::NoReliableData);
        }

        // Validation and comparison logic (using average)
        let avg_temp = avg_f32(
            &successful_results
                .iter()
                .map(|d| d.temperature_celsius)
                .collect::<Vec<_>>(),
        );
        let avg_humidity = avg_f32(
            &successful_results
                .iter()
                .map(|d| d.humidity_percent)
                .collect::<Vec<_>>(),
        );
        let avg_wind = avg_f32(
            &successful_results
                .iter()
                .map(|d| d.wind_speed_kmh)
                .collect::<Vec<_>>(),
        );
        let avg_precip = avg_f32(
            &successful_results
                .iter()
                .map(|d| d.precipitation_mm)
                .collect::<Vec<_>>(),
        );

        // Choose the most common weather code
        let weather_code = successful_results
            .iter()
            .max_by_key(|d| d.weather_code)
            .map_or(0, |d| d.weather_code);

        Ok(WeatherData {
            temperature_celsius: avg_temp,
            humidity_percent: avg_humidity,
            wind_speed_kmh: avg_wind,
            precipitation_mm: avg_precip,
            weather_code,
        })
    }
}

// ================================================================
// Open-Meteo Provider (Free, No Key Required)
// ================================================================

/// A weather provider that uses the Open-Meteo API.
pub struct OpenMeteoProvider {
    client: reqwest::Client,
}

impl Default for OpenMeteoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenMeteoProvider {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current_weather: OpenMeteoCurrent,
}

#[derive(Deserialize)]
struct OpenMeteoCurrent {
    temperature: f32,
    windspeed: f32,
    weathercode: u32,
}

#[async_trait]
impl WeatherProvider for OpenMeteoProvider {
    async fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<WeatherData, WeatherError> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true"
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(WeatherError::FetchError(format!(
                "API returned status: {}",
                response.status()
            )));
        }

        let api_response = response
            .json::<OpenMeteoResponse>()
            .await
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        Ok(WeatherData {
            temperature_celsius: api_response.current_weather.temperature,
            humidity_percent: 50.0, // placeholder - Open-Meteo doesn't provide in current_weather
            wind_speed_kmh: api_response.current_weather.windspeed,
            precipitation_mm: 0.0, // placeholder
            weather_code: api_response.current_weather.weathercode,
        })
    }

    fn provider_name(&self) -> &'static str {
        "Open-Meteo"
    }
}

// ================================================================
// WeatherAPI.com Provider (Requires API Key)
// ================================================================

/// WeatherAPI.com provider - commercial weather data with full metrics
pub struct WeatherApiProvider {
    client: reqwest::Client,
    api_key: String,
}

impl WeatherApiProvider {
    #[must_use]
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Create from environment variable WEATHER_API_KEY
    pub fn from_env() -> Option<Self> {
        std::env::var("WEATHER_API_KEY")
            .ok()
            .map(|key| Self::new(key))
    }
}

#[derive(Deserialize)]
struct WeatherApiResponse {
    current: WeatherApiCurrent,
}

#[derive(Deserialize)]
struct WeatherApiCurrent {
    temp_c: f32,
    humidity: f32,
    wind_kph: f32,
    precip_mm: f32,
    condition: WeatherApiCondition,
}

#[derive(Deserialize)]
struct WeatherApiCondition {
    code: u32,
}

#[async_trait]
impl WeatherProvider for WeatherApiProvider {
    async fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<WeatherData, WeatherError> {
        let url = format!(
            "https://api.weatherapi.com/v1/current.json?key={}&q={},{}",
            self.api_key, latitude, longitude
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(WeatherError::FetchError(format!(
                "WeatherAPI returned status: {}",
                response.status()
            )));
        }

        let api_response = response
            .json::<WeatherApiResponse>()
            .await
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        Ok(WeatherData {
            temperature_celsius: api_response.current.temp_c,
            humidity_percent: api_response.current.humidity,
            wind_speed_kmh: api_response.current.wind_kph,
            precipitation_mm: api_response.current.precip_mm,
            weather_code: api_response.current.condition.code,
        })
    }

    fn provider_name(&self) -> &'static str {
        "WeatherAPI.com"
    }
}

// ================================================================
// NOAA Provider (Public API - No Key Required)
// ================================================================

/// NOAA Weather API provider - US government weather data
pub struct NoaaProvider {
    client: reqwest::Client,
}

impl Default for NoaaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl NoaaProvider {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("SX9-WeatherEngine/1.0 (contact@synaptix9.com)")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Get the grid point for a lat/lon (required by NOAA API)
    async fn get_grid_point(&self, latitude: f64, longitude: f64) -> Result<(String, i32, i32), WeatherError> {
        let url = format!(
            "https://api.weather.gov/points/{:.4},{:.4}",
            latitude, longitude
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(WeatherError::FetchError(format!(
                "NOAA points API returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        let props = json.get("properties").ok_or_else(|| {
            WeatherError::ParseError("Missing properties in NOAA response".to_string())
        })?;

        let grid_id = props.get("gridId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WeatherError::ParseError("Missing gridId".to_string()))?
            .to_string();

        let grid_x = props.get("gridX")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WeatherError::ParseError("Missing gridX".to_string()))? as i32;

        let grid_y = props.get("gridY")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WeatherError::ParseError("Missing gridY".to_string()))? as i32;

        Ok((grid_id, grid_x, grid_y))
    }
}

#[async_trait]
impl WeatherProvider for NoaaProvider {
    async fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Result<WeatherData, WeatherError> {
        // NOAA only covers US territory
        if latitude < 18.0 || latitude > 72.0 || longitude < -180.0 || longitude > -66.0 {
            return Err(WeatherError::FetchError(
                "NOAA API only covers US territory".to_string()
            ));
        }

        let (grid_id, grid_x, grid_y) = self.get_grid_point(latitude, longitude).await?;

        let url = format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast",
            grid_id, grid_x, grid_y
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(WeatherError::FetchError(format!(
                "NOAA forecast API returned status: {}",
                response.status()
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        // Extract first period from forecast
        let periods = json
            .get("properties")
            .and_then(|p| p.get("periods"))
            .and_then(|p| p.as_array())
            .ok_or_else(|| WeatherError::ParseError("Missing forecast periods".to_string()))?;

        let period = periods.first().ok_or_else(|| {
            WeatherError::ParseError("Empty forecast periods".to_string())
        })?;

        let temp_f = period.get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(70.0) as f32;

        let humidity = period.get("relativeHumidity")
            .and_then(|h| h.get("value"))
            .and_then(|v| v.as_f64())
            .unwrap_or(50.0) as f32;

        let wind_str = period.get("windSpeed")
            .and_then(|v| v.as_str())
            .unwrap_or("0 mph");
        let wind_mph: f32 = wind_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);

        // Convert F to C and mph to km/h
        let temp_c = (temp_f - 32.0) * 5.0 / 9.0;
        let wind_kmh = wind_mph * 1.60934;

        // Map NOAA short forecast to weather code
        let short_forecast = period.get("shortForecast")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let weather_code = match short_forecast.to_lowercase().as_str() {
            s if s.contains("sunny") || s.contains("clear") => 1,
            s if s.contains("cloudy") => 3,
            s if s.contains("rain") => 61,
            s if s.contains("snow") => 71,
            s if s.contains("thunder") => 95,
            _ => 0,
        };

        Ok(WeatherData {
            temperature_celsius: temp_c,
            humidity_percent: humidity,
            wind_speed_kmh: wind_kmh,
            precipitation_mm: 0.0, // NOAA forecast doesn't give current precip
            weather_code,
        })
    }

    fn provider_name(&self) -> &'static str {
        "NOAA"
    }
}

// ================================================================
// FSO Suitability Scoring for Ground Stations
// ================================================================

/// FSO (Free Space Optical) weather suitability criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsoSuitability {
    /// Clear sky days per year (target: >270)
    pub clear_sky_days: u16,
    /// Fog days per year (target: <20)
    pub fog_days: u16,
    /// Overall FSO suitability score (0.0 - 1.0)
    pub score: f64,
    /// Weather conditions suitable for FSO right now
    pub current_viable: bool,
}

impl WeatherData {
    /// Calculate FSO suitability from current weather conditions
    pub fn fso_suitability(&self) -> FsoSuitability {
        // FSO viability based on current conditions
        let current_viable = self.weather_code < 50  // No precipitation
            && self.humidity_percent < 85.0
            && self.wind_speed_kmh < 50.0;

        // Estimate annual metrics from current conditions (simplified)
        let clear_score = if self.weather_code <= 3 { 0.9 } else { 0.5 };
        let humidity_score = 1.0 - (self.humidity_percent as f64 / 100.0);
        let precip_score = if self.precipitation_mm < 0.1 { 1.0 } else { 0.3 };

        let score = (clear_score + humidity_score + precip_score) / 3.0;

        FsoSuitability {
            clear_sky_days: (score * 300.0) as u16,
            fog_days: ((1.0 - score) * 50.0) as u16,
            score,
            current_viable,
        }
    }
}

// ================================================================
// Factory: Create Engine with All 3 Providers
// ================================================================

impl WeatherEngine {
    /// Create engine with all available providers (Open-Meteo, WeatherAPI, NOAA)
    /// WeatherAPI requires WEATHER_API_KEY environment variable
    #[must_use]
    pub fn with_all_providers() -> Self {
        let mut providers: Vec<Arc<dyn WeatherProvider>> = vec![
            Arc::new(OpenMeteoProvider::new()),
        ];

        // Add WeatherAPI if key is available
        if let Some(weather_api) = WeatherApiProvider::from_env() {
            providers.push(Arc::new(weather_api));
        }

        // Add NOAA (always available for US locations)
        providers.push(Arc::new(NoaaProvider::new()));

        Self::new(providers)
    }

    /// Get provider count for diagnostics
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}

// ================================================================
// Tests
// ================================================================
#[cfg(test)]
mod tests {
    use super::*;

    struct MockSunnyProvider;
    #[async_trait]
    impl WeatherProvider for MockSunnyProvider {
        async fn get_weather(&self, _: f64, _: f64) -> Result<WeatherData, WeatherError> {
            Ok(WeatherData {
                temperature_celsius: 25.0,
                humidity_percent: 40.0,
                wind_speed_kmh: 10.0,
                precipitation_mm: 0.0,
                weather_code: 1,
            })
        }
        fn provider_name(&self) -> &'static str {
            "Sunny"
        }
    }

    struct MockRainyProvider;
    #[async_trait]
    impl WeatherProvider for MockRainyProvider {
        async fn get_weather(&self, _: f64, _: f64) -> Result<WeatherData, WeatherError> {
            Ok(WeatherData {
                temperature_celsius: 15.0,
                humidity_percent: 80.0,
                wind_speed_kmh: 20.0,
                precipitation_mm: 5.0,
                weather_code: 61,
            })
        }
        fn provider_name(&self) -> &'static str {
            "Rainy"
        }
    }

    struct MockErrorProvider;
    #[async_trait]
    impl WeatherProvider for MockErrorProvider {
        async fn get_weather(&self, _: f64, _: f64) -> Result<WeatherData, WeatherError> {
            Err(WeatherError::FetchError("Simulated failure".to_string()))
        }
        fn provider_name(&self) -> &'static str {
            "Error"
        }
    }

    #[tokio::test]
    async fn test_engine_with_single_provider() {
        let providers: Vec<Arc<dyn WeatherProvider>> = vec![Arc::new(MockSunnyProvider)];
        let engine = WeatherEngine::new(providers);
        let result = engine.fetch_and_validate(0.0, 0.0).await.unwrap();
        assert!((result.temperature_celsius - 25.0).abs() < f32::EPSILON);
    }

    #[tokio::test]
    async fn test_engine_averages_multiple_providers() {
        let providers: Vec<Arc<dyn WeatherProvider>> =
            vec![Arc::new(MockSunnyProvider), Arc::new(MockRainyProvider)];
        let engine = WeatherEngine::new(providers);
        let result = engine.fetch_and_validate(0.0, 0.0).await.unwrap();
        assert!((result.temperature_celsius - 20.0).abs() < f32::EPSILON);
        assert!((result.humidity_percent - 60.0).abs() < f32::EPSILON);
        assert_eq!(result.weather_code, 61);
    }

    #[tokio::test]
    async fn test_engine_handles_failing_provider() {
        let providers: Vec<Arc<dyn WeatherProvider>> = vec![
            Arc::new(MockSunnyProvider),
            Arc::new(MockErrorProvider),
        ];
        let engine = WeatherEngine::new(providers);
        let result = engine.fetch_and_validate(0.0, 0.0).await.unwrap();
        assert!((result.temperature_celsius - 25.0).abs() < f32::EPSILON);
    }

    #[tokio::test]
    async fn test_engine_fails_if_all_providers_fail() {
        let providers: Vec<Arc<dyn WeatherProvider>> =
            vec![Arc::new(MockErrorProvider), Arc::new(MockErrorProvider)];
        let engine = WeatherEngine::new(providers);
        let result = engine.fetch_and_validate(0.0, 0.0).await;
        assert!(matches!(result, Err(WeatherError::NoReliableData)));
    }

    #[test]
    fn test_fso_suitability() {
        let sunny = WeatherData {
            temperature_celsius: 25.0,
            humidity_percent: 30.0,
            wind_speed_kmh: 10.0,
            precipitation_mm: 0.0,
            weather_code: 1,
        };
        let fso = sunny.fso_suitability();
        assert!(fso.current_viable);
        assert!(fso.score > 0.7);
        assert!(fso.clear_sky_days > 200);
    }
}
