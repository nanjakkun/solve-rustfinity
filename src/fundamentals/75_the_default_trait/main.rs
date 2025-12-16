/*
In this challenge, you will implement the Default trait for an AppConfig struct that has several configuration options. The struct should have the following fields and default values:

    theme (String): Default value "Light"
    notifications_enabled (bool): Default value true
    max_users (u32): Default value 100
    auto_save (bool): Default value true
    cache_size_mb (u32): Default value 512
    log_level (String): Default value "INFO"
    retry_attempts (u32): Default value 3
    timeout_seconds (u32): Default value 30

Your task is to:

    Define the AppConfig struct with all specified fields
    Implement the Default trait manually
*/

#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
    pub auto_save: bool,
    pub cache_size_mb: u32,
    pub log_level: String,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

// TODO: implement the `Default` trait for `AppConfig`
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: String::from("Light"),
            notifications_enabled: true,
            max_users: 100,
            auto_save: true,
            cache_size_mb: 512,
            log_level: String::from("INFO"),
            retry_attempts: 3,
            timeout_seconds: 30,
        }
    }
}

// Example usage
pub fn main() {
    // Create a default configuration
    let default_config = AppConfig::default();
    println!("Default Config: {:?}", default_config);

    // Create a custom configuration using ..Default::default()
    let custom_config = AppConfig {
        theme: String::from("Dark"),
        ..Default::default()
    };
    println!("Custom Config: {:?}", custom_config);
}
