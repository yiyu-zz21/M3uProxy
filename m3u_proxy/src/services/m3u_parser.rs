use crate::error::{AppError, Result};
use crate::models::Channel;
use regex::Regex;
use std::fs;

pub struct M3uParser;

impl M3uParser {
    /// 解析 M3U 文件
    pub fn parse_file(path: &str) -> Result<Vec<Channel>> {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Io(e))?;

        Self::parse_content(&content)
    }

    /// 解析 M3U 内容
    pub fn parse_content(content: &str) -> Result<Vec<Channel>> {
        let mut channels = Vec::new();

        // EXTINF 行的正则表达式
        // 格式: #EXTINF:-1 tvg-id="..." tvg-name="..." tvg-logo="..." group-title="...",频道名称
        let extinf_regex = Regex::new(
            r#"#EXTINF:-1\s+(?:tvg-id="([^"]*)"\s*)?(?:tvg-name="([^"]*)"\s*)?(?:tvg-logo="([^"]*)"\s*)?(?:group-title="([^"]*)"\s*)?,(.+)"#
        ).map_err(|e| AppError::InvalidM3U(format!("Invalid regex: {}", e)))?;

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // 跳过空行和注释（除了 EXTINF）
            if line.is_empty() || (line.starts_with('#') && !line.starts_with("#EXTINF")) {
                i += 1;
                continue;
            }

            // 如果是 EXTINF 行
            if line.starts_with("#EXTINF") {
                if let Some(caps) = extinf_regex.captures(line) {
                    // 下一行应该是 URL
                    i += 1;
                    if i < lines.len() {
                        let url = lines[i].trim();

                        if !url.is_empty() && !url.starts_with('#') {
                            let tvg_id = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                            let _tvg_name = caps.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
                            let logo = caps.get(3).map(|m| m.as_str().to_string()).filter(|s| !s.is_empty());
                            let group = caps.get(4).map(|m| m.as_str().to_string()).unwrap_or_else(|| "未分类".to_string());
                            let name = caps.get(5).map(|m| m.as_str().trim().to_string()).unwrap_or_else(|| "未命名频道".to_string());

                            // 生成唯一 ID（始终使用索引确保唯一性）
                            let id = format!("channel_{}", channels.len());

                            let stream_type = Channel::detect_stream_type(url);

                            channels.push(Channel {
                                id,
                                tvg_id,
                                name,
                                logo,
                                group,
                                url: url.to_string(),
                                stream_type,
                            });
                        }
                    }
                }
            }

            i += 1;
        }

        if channels.is_empty() {
            return Err(AppError::InvalidM3U("No valid channels found in M3U file".to_string()));
        }

        Ok(channels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extinf() {
        let content = r#"#EXTM3U x-tvg-url="https://epg-1.iill.top/epg.xml"

#EXTINF:-1 tvg-id="咪咕体育" tvg-name="咪咕体育" tvg-logo="https://epg-1.iill.top/logo/咪咕.png" group-title="•咪咕「移动」",咪咕直播 𝟜𝕂-𝟙「移动」
http://gslbserv.itv.cmvideo.cn:80/3000000010000005180/index.m3u8?channel-id=FifastbLive

#EXTINF:-1 tvg-id="CCTV1" tvg-name="CCTV1" tvg-logo="https://example.com/logo.png" group-title="央视",CCTV-1 综合
http://example.com/cctv1.m3u8
"#;

        let result = M3uParser::parse_content(content);
        assert!(result.is_ok());

        let channels = result.unwrap();
        assert_eq!(channels.len(), 2);

        assert_eq!(channels[0].tvg_id, "咪咕体育");
        assert_eq!(channels[0].name, "咪咕直播 𝟜𝕂-𝟙「移动」");
        assert_eq!(channels[0].group, "•咪咕「移动」");
    }
}
