import type { Channel, ChannelsResponse, GroupsResponse, PlayInfo, ChannelQuery } from '../types/channel';

// API 基础 URL - 直接使用后端地址
const API_BASE_URL = 'http://localhost:8006';

// 通用请求方法
async function fetchAPI<T>(url: string): Promise<T> {
  const response = await fetch(url);

  if (!response.ok) {
    throw new Error(`API 请求失败: ${response.statusText}`);
  }

  return response.json();
}

// 获取所有频道
export async function getChannels(query?: ChannelQuery): Promise<ChannelsResponse> {
  const params = new URLSearchParams();

  if (query?.group) {
    params.append('group', query.group);
  }

  if (query?.search) {
    params.append('search', query.search);
  }

  const url = `${API_BASE_URL}/api/channels${params.toString() ? `?${params.toString()}` : ''}`;
  return fetchAPI<ChannelsResponse>(url);
}

// 获取单个频道
export async function getChannelById(id: string): Promise<Channel> {
  const url = `${API_BASE_URL}/api/channels/${id}`;
  return fetchAPI<Channel>(url);
}

// 获取所有分组
export async function getGroups(): Promise<GroupsResponse> {
  const url = `${API_BASE_URL}/api/groups`;
  return fetchAPI<GroupsResponse>(url);
}

// 获取播放信息
export async function getPlayInfo(id: string): Promise<PlayInfo> {
  const url = `${API_BASE_URL}/api/play/${id}`;
  return fetchAPI<PlayInfo>(url);
}
