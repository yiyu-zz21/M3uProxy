use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub tvg_id: String,
    pub name: String,
    pub logo: Option<String>,
    pub group: String,
    pub url: String,
    pub stream_type: StreamType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StreamType {
    HLS,
    MP4,
    FLV,
    Other,
}

impl Channel {
    pub fn detect_stream_type(url: &str) -> StreamType {
        let url_lower = url.to_lowercase();
        if url_lower.contains(".m3u8") || url_lower.contains("m3u8") {
            StreamType::HLS
        } else if url_lower.ends_with(".mp4") {
            StreamType::MP4
        } else if url_lower.ends_with(".flv") || url_lower.contains("flv") {
            StreamType::FLV
        } else {
            StreamType::Other
        }
    }
}
