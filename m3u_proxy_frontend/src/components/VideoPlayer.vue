<template>
  <div class="video-player-container">
    <div class="video-wrapper" :key="playerKey">
      <video
        ref="videoElement"
        class="video-js vjs-default-skin"
        controls
        preload="auto"
        width="100%"
        height="500"
      >
        <p class="vjs-no-js">
          要查看此视频，请启用JavaScript，并考虑升级到支持HTML5视频的浏览器。
          <a href="https://videojs.com/html5-video-support/" target="_blank">支持列表</a>
        </p>
      </video>
    </div>

    <div v-if="!currentChannel" class="empty-player">
      <div class="empty-icon">📺</div>
      <p>请选择一个频道开始观看</p>
    </div>

    <div v-if="loading" class="loading-overlay">
      <div class="spinner"></div>
      <p>正在加载...</p>
    </div>

    <div v-if="error" class="error-message">
      <div class="error-icon">⚠️</div>
      <p>{{ error }}</p>
      <button @click="reload" class="retry-button">重试</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick } from 'vue';
import videojs from 'video.js';
import '@videojs/http-streaming'; // HLS 支持
import 'video.js/dist/video-js.css';
import type { Channel } from '../types/channel';
import { getPlayInfo } from '../services/api';

interface Props {
  channel: Channel | null;
}

const props = defineProps<Props>();

const videoElement = ref<HTMLVideoElement | null>(null);
const player = ref<any>(null);
const currentChannel = ref<Channel | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const playUrl = ref<string>('');
const abortController = ref<AbortController | null>(null);
const playerKey = ref(0); // 用于强制重建播放器

async function loadChannel(channel: Channel) {
  if (!channel) return;

  console.log('开始加载频道:', channel.name);

  // 取消之前的请求
  if (abortController.value) {
    abortController.value.abort();
  }
  abortController.value = new AbortController();

  loading.value = true;
  error.value = null;

  // 清空播放器引用（Vue 会自动清理 DOM）
  player.value = null;

  // 增加 playerKey，强制 Vue 重建整个 video-wrapper
  playerKey.value++;
  console.log('playerKey 更新为:', playerKey.value);

  // 等待 Vue 完成 DOM 重建
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 50));

  try {
    const playInfo = await getPlayInfo(channel.id, abortController.value.signal);

    // 检查请求是否已被取消
    if (abortController.value.signal.aborted) {
      console.log('请求已取消');
      loading.value = false;
      return;
    }

    console.log('获取播放信息成功:', playInfo.play_url);

    // 更新频道信息
    currentChannel.value = channel;
    playUrl.value = playInfo.play_url;

    // 再次等待，确保 Vue 完成 DOM 更新
    await nextTick();
    await new Promise(resolve => setTimeout(resolve, 50));

    // 确保 video 元素已经重新渲染
    if (!videoElement.value) {
      console.error('Video 元素不存在');
      error.value = 'Video 元素初始化失败';
      loading.value = false;
      return;
    }

    console.log('开始创建 videojs 实例');

    // 创建新的播放器实例（每次都重新创建以确保状态干净）
    player.value = videojs(videoElement.value, {
      controls: true,
      autoplay: true, // 启用自动播放
      preload: 'auto',
      fluid: true,
      sources: [
        {
          src: playInfo.play_url,
          type: 'application/x-mpegURL',
        },
      ],
    });

    console.log('videojs 实例创建成功');

    // 错误事件监听
    player.value.on('error', () => {
      const error = player.value.error();
      if (error) {
        console.error('播放器错误:', error.code, error.message);
      }
    });

    // 调试事件监听
    player.value.on('loadstart', () => {
      console.log('🔄 开始加载视频');
    });

    player.value.on('loadedmetadata', () => {
      console.log('✅ 元数据加载完成');
    });

    player.value.on('loadeddata', () => {
      console.log('✅ 视频数据加载完成');
    });

    player.value.on('canplay', () => {
      console.log('✅ 视频可以播放');
    });

    player.value.on('playing', () => {
      console.log('▶️ 视频正在播放');
    });

    currentChannel.value = channel;
    playUrl.value = playInfo.play_url;
  } catch (err) {
    // 如果是请求被取消，不显示错误
    if (err instanceof Error && err.name === 'AbortError') {
      return;
    }
    error.value = err instanceof Error ? err.message : '加载播放源失败';
    console.error('加载频道失败:', err);
  } finally {
    loading.value = false;
  }
}

function reload() {
  if (currentChannel.value) {
    loadChannel(currentChannel.value);
  }
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}

watch(
  () => props.channel,
  (newChannel) => {
    if (newChannel) {
      loadChannel(newChannel);
    }
  },
  { immediate: true }
);

onMounted(() => {
  if (props.channel) {
    loadChannel(props.channel);
  }
});

onBeforeUnmount(() => {
  // 取消正在进行的请求
  if (abortController.value) {
    abortController.value.abort();
  }

  // 清空播放器引用（Vue 会自动清理 DOM）
  player.value = null;
});
</script>

<style scoped>
.video-player-container {
  width: 100%;
  background: #000;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.video-wrapper {
  position: relative;
  width: 100%;
  background: #000;
}

:deep(.video-js) {
  width: 100%;
  height: 100%;
}

:deep(.vjs-default-skin) {
  color: #fff;
}

:deep(.vjs-control-bar) {
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
}

.empty-player {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 500px;
  background: #1a1a1a;
  color: #666;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty-player p {
  font-size: 18px;
}

.loading-overlay,
.error-message {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  z-index: 10;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid rgba(255, 255, 255, 0.3);
  border-top-color: #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.error-message p {
  font-size: 16px;
  margin-bottom: 16px;
}

.retry-button {
  padding: 10px 24px;
  background: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  cursor: pointer;
  transition: background 0.3s;
}

.retry-button:hover {
  background: #2980b9;
}
</style>
