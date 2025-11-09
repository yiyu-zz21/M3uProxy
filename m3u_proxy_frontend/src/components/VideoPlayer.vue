<template>
  <div class="video-player-container">
    <div class="video-wrapper">
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
import { ref, watch, onMounted, onBeforeUnmount } from 'vue';
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

async function loadChannel(channel: Channel) {
  if (!channel || !videoElement.value) return;

  loading.value = true;
  error.value = null;

  try {
    const playInfo = await getPlayInfo(channel.id);

    if (!player.value) {
      player.value = videojs(videoElement.value, {
        controls: true,
        autoplay: false,
        preload: 'auto',
        fluid: true,
        sources: [
          {
            src: playInfo.play_url,
            type: 'application/x-mpegURL',
          },
        ],
      });

      player.value.on('error', () => {
        const error = player.value.error();
        if (error) {
          console.error('播放器错误:', error);
        }
      });
    } else {
      player.value.src({
        src: playInfo.play_url,
        type: 'application/x-mpegURL',
      });
    }

    currentChannel.value = channel;
    playUrl.value = playInfo.play_url;
  } catch (err) {
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
  if (player.value) {
    player.value.dispose();
    player.value = null;
  }
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
