use crate::error::AppError;
use tracing::debug;
use url::Url;

/// M3U8 URL 重写器
pub struct M3u8Rewriter {
    proxy_base_url: String,
}

impl M3u8Rewriter {
    /// 创建新的 M3U8 重写器
    pub fn new(proxy_base_url: String) -> Self {
        Self { proxy_base_url }
    }

    /// 重写 M3U8 内容中的 URL
    ///
    /// 将 M3U8 文件中的所有 URL（包括播放列表和片段）重写为通过代理服务器访问
    pub fn rewrite_m3u8(&self, content: &str, original_url: &str) -> Result<String, AppError> {
        let mut result = String::new();

        // 解析原始 URL 以便处理相对路径
        let base_url = Url::parse(original_url)
            .map_err(|e| AppError::InvalidM3U(format!("Invalid base URL: {}", e)))?;

        for line in content.lines() {
            let trimmed = line.trim();

            // 跳过空行和注释行（除了 #EXT-X-KEY）
            if trimmed.is_empty() || (trimmed.starts_with('#') && !trimmed.starts_with("#EXT-X-KEY")) {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // 处理 #EXT-X-KEY 行中的 URI
            if trimmed.starts_with("#EXT-X-KEY") {
                let rewritten = self.rewrite_key_line(trimmed, &base_url)?;
                result.push_str(&rewritten);
                result.push('\n');
                continue;
            }

            // 处理 URL 行
            if !trimmed.starts_with('#') {
                let absolute_url = self.resolve_url(trimmed, &base_url)?;
                let proxied_url = self.create_proxy_url(&absolute_url)?;
                debug!("Rewriting URL: {} -> {}", trimmed, proxied_url);
                result.push_str(&proxied_url);
                result.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
    }

    /// 重写 #EXT-X-KEY 行中的 URI
    fn rewrite_key_line(&self, line: &str, base_url: &Url) -> Result<String, AppError> {
        if let Some(uri_start) = line.find("URI=\"") {
            let uri_start = uri_start + 5; // "URI=\"" 的长度
            if let Some(uri_end) = line[uri_start..].find('"') {
                let uri = &line[uri_start..uri_start + uri_end];
                let absolute_url = self.resolve_url(uri, base_url)?;
                let proxied_url = self.create_proxy_url(&absolute_url)?;

                let mut result = String::from(&line[..uri_start]);
                result.push_str(&proxied_url);
                result.push_str(&line[uri_start + uri_end..]);

                return Ok(result);
            }
        }
        Ok(line.to_string())
    }

    /// 将相对 URL 转换为绝对 URL
    fn resolve_url(&self, url: &str, base_url: &Url) -> Result<String, AppError> {
        // 如果已经是绝对 URL，直接返回
        if url.starts_with("http://") || url.starts_with("https://") {
            return Ok(url.to_string());
        }

        // 解析为绝对 URL
        let absolute = base_url
            .join(url)
            .map_err(|e| AppError::InvalidM3U(format!("Failed to resolve URL: {}", e)))?;

        Ok(absolute.to_string())
    }

    /// 创建代理 URL
    fn create_proxy_url(&self, original_url: &str) -> Result<String, AppError> {
        // 判断是播放列表还是片段
        let endpoint = if original_url.ends_with(".m3u8") || original_url.contains(".m3u8?") {
            "playlist"
        } else {
            "segment"
        };

        // URL 编码原始 URL
        let encoded_url = urlencoding::encode(original_url);

        // 使用相对路径，让浏览器基于当前页面的 origin 来请求
        // 这样可以通过 Vite 代理或其他前端代理转发到后端
        Ok(format!("/api/proxy/{}?url={}", endpoint, encoded_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_simple_m3u8() {
        let rewriter = M3u8Rewriter::new("http://localhost:8006".to_string());
        let content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXTINF:10.0,
segment1.ts
#EXTINF:10.0,
segment2.ts
#EXT-X-ENDLIST"#;

        let result = rewriter.rewrite_m3u8(content, "http://example.com/playlist.m3u8").unwrap();

        assert!(result.contains("/api/proxy/segment?url="));
        assert!(result.contains("segment1.ts"));
        assert!(result.contains("segment2.ts"));
    }

    #[test]
    fn test_resolve_relative_url() {
        let rewriter = M3u8Rewriter::new("http://localhost:8006".to_string());
        let base_url = Url::parse("http://example.com/path/playlist.m3u8").unwrap();

        let result = rewriter.resolve_url("segment.ts", &base_url).unwrap();
        assert_eq!(result, "http://example.com/path/segment.ts");

        let result = rewriter.resolve_url("/other/segment.ts", &base_url).unwrap();
        assert_eq!(result, "http://example.com/other/segment.ts");

        let result = rewriter.resolve_url("http://other.com/segment.ts", &base_url).unwrap();
        assert_eq!(result, "http://other.com/segment.ts");
    }
}
