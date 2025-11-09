<template>
  <div id="app">
    <header class="app-header">
      <div class="header-content">
        <h1>📺 IPTV Player</h1>
        <p class="subtitle">基于 Rust + Axum 的 M3U8 代理播放器</p>
      </div>
    </header>

    <main class="app-main">
      <aside class="sidebar">
        <ChannelList @channel-select="handleChannelSelect" />
      </aside>

      <section class="player-section">
        <VideoPlayer :channel="selectedChannel" />
      </section>
    </main>

    <footer class="app-footer">
      <p>© 2025 M3U Proxy | 基于 Rust Axum + Vue 3 构建</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import ChannelList from './components/ChannelList.vue';
import VideoPlayer from './components/VideoPlayer.vue';
import type { Channel } from './types/channel';

const selectedChannel = ref<Channel | null>(null);

function handleChannelSelect(channel: Channel) {
  selectedChannel.value = channel;
}
</script>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
}

.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px 40px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
}

.header-content h1 {
  margin: 0 0 8px 0;
  font-size: 32px;
  font-weight: 700;
}

.subtitle {
  margin: 0;
  font-size: 14px;
  opacity: 0.9;
}

.app-main {
  flex: 1;
  display: flex;
  gap: 20px;
  padding: 20px;
  max-width: 1920px;
  width: 100%;
  margin: 0 auto;
  box-sizing: border-box;
  min-height: 0;
}

.sidebar {
  width: 400px;
  flex-shrink: 0;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.player-section {
  flex: 1;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.app-footer {
  background: #2c3e50;
  color: white;
  text-align: center;
  padding: 20px;
  font-size: 14px;
  flex-shrink: 0;
}

.app-footer p {
  margin: 0;
}

@media (max-width: 1200px) {
  .app-main {
    flex-direction: column;
  }

  .sidebar {
    width: 100%;
    max-height: 400px;
  }
}

@media (max-width: 768px) {
  .app-header {
    padding: 15px 20px;
  }

  .header-content h1 {
    font-size: 24px;
  }

  .subtitle {
    font-size: 12px;
  }

  .app-main {
    padding: 10px;
    gap: 10px;
  }

  .sidebar {
    max-height: 300px;
  }
}
</style>
