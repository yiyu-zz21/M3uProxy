<template>
  <div class="channel-list">
    <!-- 搜索和筛选 -->
    <div class="controls">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="搜索频道..."
        class="search-input"
        @input="handleSearch"
      />

      <select v-model="selectedGroup" class="group-select" @change="handleGroupChange">
        <option value="">所有分组</option>
        <option v-for="group in groups" :key="group" :value="group">
          {{ group }}
        </option>
      </select>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading">加载中...</div>

    <!-- 错误信息 -->
    <div v-if="error" class="error">{{ error }}</div>

    <!-- 频道网格 -->
    <div v-if="!loading && !error" class="channels-grid">
      <div
        v-for="channel in filteredChannels"
        :key="channel.id"
        class="channel-card"
        :class="{ active: selectedChannelId === channel.id }"
        @click="selectChannel(channel)"
      >
        <div class="channel-logo">
          <img
            v-if="channel.logo && !imageErrors[channel.id]"
            :src="channel.logo"
            :alt="channel.name"
            @error="handleImageError(channel.id)"
          />
          <div v-else class="logo-placeholder">{{ channel.name.charAt(0) }}</div>
        </div>
        <div class="channel-info">
          <div class="channel-name">{{ channel.name }}</div>
          <div class="channel-group">{{ channel.group }}</div>
          <div class="channel-type">{{ channel.stream_type }}</div>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-if="!loading && !error && filteredChannels.length === 0" class="empty">
      没有找到频道
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import type { Channel, GroupsResponse } from '../types/channel';
import { getChannels, getGroups } from '../services/api';

// Props
interface Props {
  onChannelSelect?: (channel: Channel) => void;
}

const props = defineProps<Props>();

// 响应式数据
const channels = ref<Channel[]>([]);
const groups = ref<string[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const searchQuery = ref('');
const selectedGroup = ref('');
const selectedChannelId = ref<string | null>(null);
const imageErrors = ref<Record<string, boolean>>({});

// 计算属性：过滤后的频道
const filteredChannels = computed(() => {
  let result = channels.value;

  // 按分组筛选
  if (selectedGroup.value) {
    result = result.filter(c => c.group === selectedGroup.value);
  }

  // 按搜索词筛选
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(c =>
      c.name.toLowerCase().includes(query) ||
      c.tvg_id.toLowerCase().includes(query)
    );
  }

  return result;
});

// 方法
async function loadChannels() {
  loading.value = true;
  error.value = null;

  try {
    const query: any = {};
    if (selectedGroup.value) {
      query.group = selectedGroup.value;
    }

    const response = await getChannels(query);
    channels.value = response.channels;
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载频道失败';
  } finally {
    loading.value = false;
  }
}

async function loadGroups() {
  try {
    const response: GroupsResponse = await getGroups();
    groups.value = response.groups;
  } catch (err) {
    console.error('加载分组失败:', err);
  }
}

function selectChannel(channel: Channel) {
  selectedChannelId.value = channel.id;
  if (props.onChannelSelect) {
    props.onChannelSelect(channel);
  }
}

function handleSearch() {
  // 搜索时清除分组选择
  selectedGroup.value = '';
}

function handleGroupChange() {
  // 筛选分组时清除搜索
  searchQuery.value = '';
}

function handleImageError(channelId: string) {
  imageErrors.value[channelId] = true;
}

// 生命周期
onMounted(async () => {
  await Promise.all([loadChannels(), loadGroups()]);
});
</script>

<style scoped>
.channel-list {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.controls {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.search-input,
.group-select {
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.search-input {
  flex: 1;
}

.group-select {
  width: 200px;
}

.loading,
.error,
.empty {
  text-align: center;
  padding: 40px;
  color: #666;
}

.error {
  color: #e74c3c;
}

.channels-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 16px;
  overflow-y: auto;
  overflow-x: hidden;
  flex: 1;
  padding-right: 8px;
}

.channels-grid::-webkit-scrollbar {
  width: 8px;
}

.channels-grid::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.channels-grid::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 4px;
}

.channels-grid::-webkit-scrollbar-thumb:hover {
  background: #555;
}

.channel-card {
  display: flex;
  gap: 12px;
  padding: 16px;
  border: 2px solid #eee;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
  background: white;
}

.channel-card:hover {
  border-color: #3498db;
  box-shadow: 0 2px 8px rgba(52, 152, 219, 0.2);
}

.channel-card.active {
  border-color: #3498db;
  background: #f0f8ff;
}

.channel-logo {
  width: 60px;
  height: 60px;
  flex-shrink: 0;
}

.channel-logo img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  border-radius: 4px;
}

.logo-placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: bold;
  border-radius: 4px;
}

.channel-info {
  flex: 1;
  min-width: 0;
}

.channel-name {
  font-weight: bold;
  font-size: 16px;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.channel-group {
  font-size: 12px;
  color: #666;
  margin-bottom: 2px;
}

.channel-type {
  display: inline-block;
  padding: 2px 8px;
  background: #ecf0f1;
  border-radius: 12px;
  font-size: 11px;
  color: #7f8c8d;
}
</style>
