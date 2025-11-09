use axum::{
    extract::{Query, State},
    response::Response,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

use crate::{error::AppError, services::proxy::ProxyService};

/// 视频片段代理状态
#[derive(Clone)]
pub struct SegmentState {
    pub proxy: Arc<ProxyService>,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct SegmentQuery {
    url: String,
}

/// 代理视频片段
///
/// GET /api/proxy/segment?url={encoded_url}
///
/// 直接代理 TS 视频片段或其他媒体文件
pub async fn proxy_segment(
    State(state): State<SegmentState>,
    Query(query): Query<SegmentQuery>,
) -> Result<Response, AppError> {
    info!("Proxying segment: {}", query.url);

    // 使用流式代理来处理视频片段
    let response = state.proxy.proxy_stream(&query.url).await?;

    Ok(response)
}
