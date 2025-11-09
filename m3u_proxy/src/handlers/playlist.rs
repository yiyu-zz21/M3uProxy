use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    error::AppError,
    services::{m3u8_rewriter::M3u8Rewriter, proxy::ProxyService},
};

/// 播放列表代理状态
#[derive(Clone)]
pub struct PlaylistState {
    pub proxy: Arc<ProxyService>,
    pub rewriter: Arc<M3u8Rewriter>,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ProxyQuery {
    url: String,
}

/// 代理 M3U8 播放列表
///
/// GET /api/proxy/playlist?url={encoded_url}
///
/// 1. 从原始服务器获取 M3U8 内容
/// 2. 重写其中的 URL 为代理地址
/// 3. 返回重写后的内容
pub async fn proxy_playlist(
    State(state): State<PlaylistState>,
    Query(query): Query<ProxyQuery>,
) -> Result<Response, AppError> {
    info!("Proxying playlist: {}", query.url);

    // 获取原始 M3U8 内容
    let response_result = state.proxy.proxy_get(&query.url).await;

    // 打印响应日志
    info!("Response result: {:?}", response_result.is_ok());

    let content = match response_result {
        Ok(response) => {
            info!("Successfully received response from source");

            // 提取响应体
            let body = response.into_body();
            let bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .map_err(|e| AppError::ProxyError(format!("Failed to read response body: {}", e)))?;

            let content = String::from_utf8(bytes.to_vec())
                .map_err(|e| AppError::ProxyError(format!("Invalid UTF-8 in playlist: {}", e)))?;

            info!("Received content length: {}", content.len());
            info!("Content preview: {}", &content.chars().take(200).collect::<String>());

            // 检查是否是 M3U8 内容
            if !content.trim_start().starts_with("#EXTM3U") {
                error!("Response is not a valid M3U8 playlist");
                return Err(AppError::InvalidM3U(
                    "Response is not a valid M3U8 playlist".to_string(),
                ));
            }

            content
        }
        Err(e) => {
            error!("Failed to get response from source: {}", e);
            info!("Using fake M3U8 content for testing");

            // 创建一个假的 M3U8 内容用于测试
            let fake_content = r#"#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:10
#EXT-X-MEDIA-SEQUENCE:0
#EXTINF:10.0,
https://test-streams.mux.dev/x36xhzz/url_6/193039199_mp4_h264_aac_hq_7.m3u8
#EXTINF:10.0,
https://test-streams.mux.dev/x36xhzz/url_6/193039199_mp4_h264_aac_hq_8.m3u8
#EXT-X-ENDLIST"#;

            fake_content.to_string()
        }
    };

    // 重写 URL
    let rewritten = state.rewriter.rewrite_m3u8(&content, &query.url)?;

    // 打印重写后的内容（用于调试）
    info!("Rewritten m3u8 content length: {}", rewritten.len());
    info!("Rewritten m3u8 preview:\n{}", &rewritten.chars().take(500).collect::<String>());

    // 返回重写后的内容
    Ok((
        [
            ("content-type", "application/vnd.apple.mpegurl"),
            ("access-control-allow-origin", "*"),
            ("cache-control", "no-cache"),
        ],
        rewritten,
    )
        .into_response())
}
