use anyhow::{Context, Result};
use std::env;

// CONFIGURATION STRUCT
/// The #[derive(...)] attribute automatically implements common traits:
/// - Clone: Creates a deep copy of the struct
#[derive(Debug, Clone)]
pub struct Config {
    /// The Ollama model to use .
    pub model: String,

    /// Ollama server URL (default: http://localhost:11434)
    pub ollama_host: String,

    /// Temperature for LLM responses (0.0 = deterministic, 1.0 = creative)
    /// Lower values produce more focused, factual responses
    pub temperature: f32,

    /// Maximum number of search results to analyze
    pub max_search_results: usize,

    /// Log level for the application
    pub log_level: String,
}

// DEFAULT IMPLEMENTATION
impl Default for Config {
    fn default() -> Self {
        Self {
            // Use a common, capable model as default
            model: "llama3.2".to_string(),

            // Standard Ollama default port
            ollama_host: "http://localhost:11434".to_string(),

            // Moderate temperature - balanced between creativity and focus
            temperature: 0.7,

            // Analyze top 5 search results by default
            max_search_results: 5,

            // Info level logging by default
            log_level: "info".to_string(),
        }
    }
}
// CONFIGURATION LOADING
impl Config {

    pub fn from_env() -> Result<Self> {

        let _ = dotenvy::dotenv();

        // Start with default values
        let mut config = Config::default();

        if let Ok(val) = env::var("OLLAMA_MODEL") {
            config.model = val;
        }

        if let Ok(val) = env::var("OLLAMA_API_BASE_URL") {
            config.ollama_host = val;
        }

        // Parse temperature from string to f32
        if let Ok(val) = env::var("TEMPERATURE") {
            config.temperature = val
                .parse()
                .context("TEMPERATURE must be a valid floating-point number (e.g., 0.7)")?;
        }

        if let Ok(val) = env::var("MAX_SEARCH_RESULTS") {
            config.max_search_results = val
                .parse()
                .context("MAX_SEARCH_RESULTS must be a valid positive integer")?;
        }

        if let Ok(val) = env::var("RUST_LOG") {
            config.log_level = val;
        }

        Ok(config)
    }

    /// Validate the configuration.
    
    /// This ensures all values are within acceptable ranges before the agent starts.
    pub fn validate(&self) -> Result<()> {
        // Temperature must be between 0 and 2 (OpenAI/Ollama range)
        if !(0.0..=2.0).contains(&self.temperature) {
            anyhow::bail!(
                "Temperature must be between 0.0 and 2.0, got: {}",
                self.temperature
            );
        }

        if self.max_search_results == 0 {
            anyhow::bail!("MAX_SEARCH_RESULTS must be at least 1");
        }

        if self.model.is_empty() {
            anyhow::bail!("OLLAMA_MODEL cannot be empty");
        }

        Ok(())
    }
}

// UNIT TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.model, "llama3.2");
        assert_eq!(config.ollama_host, "http://localhost:11434");
        assert!((config.temperature - 0.7).abs() < f32::EPSILON);
        assert_eq!(config.max_search_results, 5);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_temperature() {
        let mut config = Config::default();
        config.temperature = 3.0; // Invalid: above 2.0
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_search_results() {
        let mut config = Config::default();
        config.max_search_results = 0; // Invalid: must be at least 1
        assert!(config.validate().is_err());
    }
}