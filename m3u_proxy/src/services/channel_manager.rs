use crate::error::{AppError, Result};
use crate::models::Channel;
use crate::services::M3uParser;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChannelManager {
    channels: Arc<RwLock<Vec<Channel>>>,
}

impl ChannelManager {
    /// 创建新的频道管理器
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 从 M3U 文件加载频道
    pub fn load_from_file(&self, path: &str) -> Result<usize> {
        let parsed_channels = M3uParser::parse_file(path)?;
        let count = parsed_channels.len();

        let mut channels = self.channels.write();
        *channels = parsed_channels;

        tracing::info!("Loaded {} channels from {}", count, path);
        Ok(count)
    }

    /// 获取所有频道
    pub fn get_all_channels(&self) -> Vec<Channel> {
        self.channels.read().clone()
    }

    /// 根据 ID 获取频道
    pub fn get_channel_by_id(&self, id: &str) -> Result<Channel> {
        let channels = self.channels.read();
        channels
            .iter()
            .find(|c| c.id == id)
            .cloned()
            .ok_or_else(|| AppError::ChannelNotFound(id.to_string()))
    }

    /// 根据分组筛选频道
    pub fn get_channels_by_group(&self, group: &str) -> Vec<Channel> {
        let channels = self.channels.read();
        channels
            .iter()
            .filter(|c| c.group == group)
            .cloned()
            .collect()
    }

    /// 搜索频道（按名称）
    pub fn search_channels(&self, query: &str) -> Vec<Channel> {
        let channels = self.channels.read();
        let query_lower = query.to_lowercase();

        channels
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.tvg_id.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// 获取所有分组
    pub fn get_all_groups(&self) -> Vec<String> {
        let channels = self.channels.read();
        let mut groups: Vec<String> = channels
            .iter()
            .map(|c| c.group.clone())
            .collect();

        groups.sort();
        groups.dedup();
        groups
    }

    /// 获取频道总数
    pub fn get_channel_count(&self) -> usize {
        self.channels.read().len()
    }

    /// 重新加载频道列表
    pub fn reload(&self, path: &str) -> Result<usize> {
        self.load_from_file(path)
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StreamType;

    #[test]
    fn test_channel_manager() {
        let manager = ChannelManager::new();

        // 手动添加测试频道
        let test_channels = vec![
            Channel {
                id: "test1".to_string(),
                tvg_id: "test1".to_string(),
                name: "测试频道1".to_string(),
                logo: None,
                group: "测试组".to_string(),
                url: "http://example.com/test1.m3u8".to_string(),
                stream_type: StreamType::HLS,
            },
            Channel {
                id: "test2".to_string(),
                tvg_id: "test2".to_string(),
                name: "测试频道2".to_string(),
                logo: None,
                group: "测试组".to_string(),
                url: "http://example.com/test2.m3u8".to_string(),
                stream_type: StreamType::HLS,
            },
        ];

        *manager.channels.write() = test_channels;

        assert_eq!(manager.get_channel_count(), 2);
        assert!(manager.get_channel_by_id("test1").is_ok());
        assert_eq!(manager.get_channels_by_group("测试组").len(), 2);
    }
}
