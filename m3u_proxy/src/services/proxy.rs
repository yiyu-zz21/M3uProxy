use axum::{
    body::Body,
    http::{HeaderMap, HeaderValue},
    response::Response,
};
use reqwest::Client;
use std::time::Duration;
use tracing::info;

use crate::error::AppError;

/// HTTP 代理服务
pub struct ProxyService {
    client: Client,
}

impl ProxyService {
    /// 创建新的代理服务实例
    pub fn new(timeout: u64) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|e| AppError::ProxyError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client })
    }

    /// 代理 GET 请求
    pub async fn proxy_get(&self, url: &str) -> Result<Response, AppError> {
        info!("Proxying GET request to: {}", url);

        // 发送请求
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::ProxyError(format!("Failed to fetch URL: {}", e)))?;

        // 获取状态码
        let status = response.status();

        // 构建响应头
        let mut headers = HeaderMap::new();

        // 复制重要的响应头
        for (key, value) in response.headers() {
            let key_str = key.as_str();
            // 跳过某些不应该转发的头
            if key_str == "transfer-encoding"
                || key_str == "connection"
                || key_str == "keep-alive" {
                continue;
            }

            if let Ok(value) = HeaderValue::from_bytes(value.as_bytes()) {
                headers.insert(key.clone(), value);
            }
        }

        // 添加 CORS 头
        headers.insert(
            "access-control-allow-origin",
            HeaderValue::from_static("*"),
        );
        headers.insert(
            "access-control-allow-methods",
            HeaderValue::from_static("GET, POST, OPTIONS"),
        );
        headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_static("*"),
        );

        // 获取响应体
        let bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::ProxyError(format!("Failed to read response body: {}", e)))?;

        // 构建响应
        let mut response = Response::new(Body::from(bytes));
        *response.status_mut() = status;
        *response.headers_mut() = headers;

        Ok(response)
    }

    /// 代理流式请求（用于视频片段）
    pub async fn proxy_stream(&self, url: &str) -> Result<Response, AppError> {
        info!("Proxying stream request to: {}", url);

        // 发送请求
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::ProxyError(format!("Failed to fetch stream: {}", e)))?;

        // 获取状态码
        let status = response.status();

        // 构建响应头
        let mut headers = HeaderMap::new();

        // 复制响应头
        for (key, value) in response.headers() {
            let key_str = key.as_str();
            if key_str == "transfer-encoding"
                || key_str == "connection"
                || key_str == "keep-alive" {
                continue;
            }

            if let Ok(value) = HeaderValue::from_bytes(value.as_bytes()) {
                headers.insert(key.clone(), value);
            }
        }

        // 添加 CORS 头
        headers.insert(
            "access-control-allow-origin",
            HeaderValue::from_static("*"),
        );

        // 将 reqwest 的流转换为 axum 的 Body
        let stream = response.bytes_stream();
        let body = Body::from_stream(stream);

        // 构建响应
        let mut response = Response::new(body);
        *response.status_mut() = status;
        *response.headers_mut() = headers;

        Ok(response)
    }
}
