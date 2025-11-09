mod config;
mod error;
mod handlers;
mod models;
mod services;

use axum::{routing::get, Router};
use config::Config;
use handlers::{
    get_channel_by_id, get_channels, get_groups, get_play_info, play_stream, proxy_playlist,
    proxy_segment, AppState, PlayState, PlaylistState, SegmentState,
};
use services::{ChannelManager, M3u8Rewriter, ProxyService};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "m3u_proxy=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::default();

    // 初始化频道管理器并加载 M3U 文件
    let channel_manager = Arc::new(ChannelManager::new());
    match channel_manager.load_from_file(&config.m3u_path) {
        Ok(count) => {
            tracing::info!("Successfully loaded {} channels from {}", count, config.m3u_path);
        }
        Err(e) => {
            tracing::error!("Failed to load M3U file: {}", e);
            std::process::exit(1);
        }
    }

    // 初始化代理服务
    let proxy_service = Arc::new(
        ProxyService::new(config.request_timeout).expect("Failed to create proxy service"),
    );

    // 初始化 M3U8 重写器
    let proxy_base_url = format!("http://{}:{}", config.host, config.port);
    let m3u8_rewriter = Arc::new(M3u8Rewriter::new(proxy_base_url));

    // 创建应用状态
    let channel_state = AppState {
        channel_manager: channel_manager.clone(),
    };

    let play_state = PlayState {
        channel_manager: channel_manager.clone(),
        rewriter: m3u8_rewriter.clone(),
    };

    let playlist_state = PlaylistState {
        proxy: proxy_service.clone(),
        rewriter: m3u8_rewriter.clone(),
    };

    let segment_state = SegmentState {
        proxy: proxy_service.clone(),
    };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    // 频道管理路由
    let channel_routes = Router::new()
        .route("/api/channels", get(get_channels))
        .route("/api/channels/:id", get(get_channel_by_id))
        .route("/api/groups", get(get_groups))
        .with_state(channel_state);

    // 播放路由
    let play_routes = Router::new()
        .route("/api/play/:id", get(get_play_info))
        .route("/api/play/:id/stream", get(play_stream))
        .with_state(play_state);

    // 代理路由
    let playlist_routes = Router::new()
        .route("/api/proxy/playlist", get(proxy_playlist))
        .with_state(playlist_state);

    let segment_routes = Router::new()
        .route("/api/proxy/segment", get(proxy_segment))
        .with_state(segment_state);

    // 合并所有路由
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(channel_routes)
        .merge(play_routes)
        .merge(playlist_routes)
        .merge(segment_routes)
        .layer(cors);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}
