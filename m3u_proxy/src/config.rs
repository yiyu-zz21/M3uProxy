use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_m3u_path")]
    pub m3u_path: String,

    #[serde(default = "default_cache_enabled")]
    pub cache_enabled: bool,

    #[serde(default = "default_cache_ttl_playlist")]
    pub cache_ttl_playlist: u64,

    #[serde(default = "default_cache_ttl_segment")]
    pub cache_ttl_segment: u64,

    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: usize,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8006
}

fn default_m3u_path() -> String {
    "./Gather.m3u".to_string()
}

fn default_cache_enabled() -> bool {
    true
}

fn default_cache_ttl_playlist() -> u64 {
    300
}

fn default_cache_ttl_segment() -> u64 {
    600
}

fn default_request_timeout() -> u64 {
    30
}

fn default_max_concurrent() -> usize {
    100
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            m3u_path: default_m3u_path(),
            cache_enabled: default_cache_enabled(),
            cache_ttl_playlist: default_cache_ttl_playlist(),
            cache_ttl_segment: default_cache_ttl_segment(),
            request_timeout: default_request_timeout(),
            max_concurrent: default_max_concurrent(),
        }
    }
}
