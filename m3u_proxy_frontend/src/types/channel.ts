// 频道数据类型
export interface Channel {
  id: string;
  tvg_id: string;
  name: string;
  logo?: string;
  group: string;
  url: string;
  stream_type: 'HLS' | 'MP4' | 'FLV' | 'Other';
}

// 频道列表响应
export interface ChannelsResponse {
  total: number;
  channels: Channel[];
}

// 分组列表响应
export interface GroupsResponse {
  groups: string[];
}

// 播放信息
export interface PlayInfo {
  id: string;
  name: string;
  logo?: string;
  group: string;
  stream_type: string;
  play_url: string;
  original_url: string;
}

// API 查询参数
export interface ChannelQuery {
  group?: string;
  search?: string;
}
