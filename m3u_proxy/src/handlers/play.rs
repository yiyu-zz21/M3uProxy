use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

use crate::{
    error::AppError,
    services::{channel_manager::ChannelManager, m3u8_rewriter::M3u8Rewriter},
};

/// 频道播放处理器状态
#[derive(Clone)]
pub struct PlayState {
    pub channel_manager: Arc<ChannelManager>,
    pub rewriter: Arc<M3u8Rewriter>,
}

/// 获取频道播放信息
///
/// GET /api/play/{channel_id}
///
/// 返回频道的播放信息，包括代理后的播放地址
pub async fn get_play_info(
    State(state): State<PlayState>,
    Path(channel_id): Path<String>,
) -> Result<Response, AppError> {
    info!("Getting play info for channel: {}", channel_id);

    // 获取频道信息
    let channel = state
        .channel_manager
        .get_channel_by_id(&channel_id)?;

    // 根据流类型返回不同的播放信息
    let play_url = match channel.stream_type {
        crate::models::channel::StreamType::HLS => {
            // HLS 流需要通过代理，使用相对路径
            let encoded_url = urlencoding::encode(&channel.url);
            format!("/api/proxy/playlist?url={}", encoded_url)
        }
        _ => {
            // 其他类型直接返回原始 URL
            channel.url.clone()
        }
    };

    let response = json!({
        "id": channel.id,
        "name": channel.name,
        "logo": channel.logo,
        "group": channel.group,
        "stream_type": format!("{:?}", channel.stream_type),
        "play_url": play_url,
        "original_url": channel.url,
    });

    Ok(Json(response).into_response())
}

/// 直接播放频道（重定向到代理地址）
///
/// GET /api/play/{channel_id}/stream
///
/// 直接返回流数据，适合直接在 video 标签中使用
pub async fn play_stream(
    State(state): State<PlayState>,
    Path(channel_id): Path<String>,
) -> Result<Response, AppError> {
    info!("Playing stream for channel: {}", channel_id);

    // 获取频道信息
    let channel = state
        .channel_manager
        .get_channel_by_id(&channel_id)?;

    // 根据流类型处理
    match channel.stream_type {
        crate::models::channel::StreamType::HLS => {
            // HLS 流重定向到播放列表代理
            let encoded_url = urlencoding::encode(&channel.url);
            let redirect_url = format!("/api/proxy/playlist?url={}", encoded_url);

            Ok((
                StatusCode::TEMPORARY_REDIRECT,
                [("Location", redirect_url.as_str())],
            )
                .into_response())
        }
        _ => {
            // 其他类型重定向到原始 URL
            Ok((
                StatusCode::TEMPORARY_REDIRECT,
                [("Location", channel.url.as_str())],
            )
                .into_response())
        }
    }
}
