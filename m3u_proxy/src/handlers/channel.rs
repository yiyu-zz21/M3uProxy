use crate::error::Result;
use crate::models::Channel;
use crate::services::ChannelManager;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub channel_manager: std::sync::Arc<ChannelManager>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelQuery {
    pub group: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChannelsResponse {
    pub total: usize,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Serialize)]
pub struct GroupsResponse {
    pub groups: Vec<String>,
}

/// 获取所有频道列表（支持分组和搜索过滤）
pub async fn get_channels(
    State(state): State<AppState>,
    Query(query): Query<ChannelQuery>,
) -> Result<Json<ChannelsResponse>> {
    let channels = if let Some(group) = query.group {
        // 按分组筛选
        state.channel_manager.get_channels_by_group(&group)
    } else if let Some(search) = query.search {
        // 按名称搜索
        state.channel_manager.search_channels(&search)
    } else {
        // 获取所有频道
        state.channel_manager.get_all_channels()
    };

    let total = channels.len();

    Ok(Json(ChannelsResponse { total, channels }))
}

/// 根据 ID 获取单个频道详情
pub async fn get_channel_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Channel>> {
    let channel = state.channel_manager.get_channel_by_id(&id)?;
    Ok(Json(channel))
}

/// 获取所有分组列表
pub async fn get_groups(State(state): State<AppState>) -> Result<Json<GroupsResponse>> {
    let groups = state.channel_manager.get_all_groups();
    Ok(Json(GroupsResponse { groups }))
}
