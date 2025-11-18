# 速率限制 (Rate Limiting) Cheat Sheet

## 速率限制概念

| 概念 | 說明 | 目的 |
|------|------|------|
| **速率限制** | 限制請求頻率 | 防止濫用和 DDoS |
| **令牌桶** | 令牌桶演算法 | 允許突發請求 |
| **滑動視窗** | 滑動時間視窗 | 平滑限制 |
| **固定視窗** | 固定時間視窗 | 簡單實現 |

## 速率限制策略

| 策略 | 說明 | 適用場景 |
|------|------|----------|
| **IP 限制** | 基於 IP 地址 | 防止單一 IP 濫用 |
| **用戶限制** | 基於用戶 ID | API 用戶限制 |
| **端點限制** | 基於 API 端點 | 不同端點不同限制 |
| **全域限制** | 全站限制 | 整體流量控制 |

## Tower Governor 配置

| 參數 | 說明 | 預設值 | 建議值 |
|------|------|--------|--------|
| `per_second` | 每秒允許請求數 | - | 100 |
| `burst_size` | 突發請求數 | - | 200 |
| `key_extractor` | 鍵提取器 | IP | IP/User ID |

## Tower Governor 實作

| 操作 | 方法 | 範例 |
|------|------|------|
| 建立配置 | `GovernorConfigBuilder` | `GovernorConfigBuilder::default()` |
| 設定速率 | `.per_second(n)` | `.per_second(100)` |
| 設定突發 | `.burst_size(n)` | `.burst_size(200)` |
| 建立層 | `GovernorLayer::new()` | `GovernorLayer::new(config)` |

## NGINX 速率限制

| 指令 | 說明 | 範例 |
|------|------|------|
| `limit_req_zone` | 定義限制區域 | `limit_req_zone $binary_remote_addr zone=api_limit:10m rate=100r/s;` |
| `limit_req` | 應用限制 | `limit_req zone=api_limit burst=200 nodelay;` |
| `limit_req_status` | 限制時狀態碼 | `limit_req_status 429;` |
| `limit_req_log_level` | 日誌級別 | `limit_req_log_level warn;` |

## 速率限制層級

| 層級 | 位置 | 技術 | 優勢 |
|------|------|------|------|
| **應用層** | Rust 應用 | Tower Governor | 精細控制 |
| **代理層** | NGINX | limit_req | 早期攔截 |
| **網路層** | 防火牆 | iptables | 最早期攔截 |

## 速率限制演算法

| 演算法 | 說明 | 優點 | 缺點 |
|--------|------|------|------|
| **令牌桶** | 定期添加令牌 | 允許突發 | 實現複雜 |
| **漏桶** | 固定速率流出 | 平滑流量 | 不允許突發 |
| **滑動視窗** | 滑動時間視窗 | 精確控制 | 記憶體開銷 |
| **固定視窗** | 固定時間視窗 | 簡單實現 | 邊界問題 |

## 速率限制配置範例

### Tower Governor
```rust
let config = Box::new(
    GovernorConfigBuilder::default()
        .per_second(100)      // 每秒 100 請求
        .burst_size(200)      // 允許突發 200 請求
        .finish()
        .unwrap(),
);
let layer = GovernorLayer::new(config);
```

### NGINX
```nginx
# 定義限制區域
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=100r/s;

# 應用限制
limit_req zone=api_limit burst=200 nodelay;
```

## 速率限制最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **分層限制** | 多層速率限制 | 應用層 + 代理層 |
| **不同端點不同限制** | 根據端點重要性 | 重要端點更寬鬆 |
| **白名單機制** | 允許特定 IP/用戶 | 內部服務不受限 |
| **監控和告警** | 追蹤限制觸發 | Prometheus 指標 |
| **漸進式限制** | 逐步降低速率 | 避免突然拒絕 |

## 速率限制指標

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 限制觸發率 | 被限制的請求比例 | < 1% |
| 平均請求速率 | 平均每秒請求數 | 監控趨勢 |
| 突發請求數 | 突發請求數量 | 監控峰值 |
| 429 錯誤率 | 速率限制錯誤 | < 0.5% |

## 速率限制響應

| 狀態碼 | 說明 | 標頭 |
|--------|------|------|
| `429 Too Many Requests` | 超過限制 | `Retry-After: 60` |
| `503 Service Unavailable` | 服務暫時不可用 | `Retry-After: 30` |

## 速率限制配置建議

| 場景 | 每秒請求數 | 突發請求數 | 理由 |
|------|------------|------------|------|
| **公開 API** | 10-50 | 20-100 | 防止濫用 |
| **認證 API** | 5-10 | 10-20 | 防止暴力破解 |
| **內部 API** | 100-500 | 200-1000 | 較寬鬆 |
| **管理 API** | 20-100 | 50-200 | 中等限制 |

## 白名單和黑名單

| 機制 | 說明 | 實作 |
|------|------|------|
| **IP 白名單** | 允許特定 IP | NGINX `geo` 模組 |
| **用戶白名單** | 允許特定用戶 | 應用層檢查 |
| **IP 黑名單** | 拒絕特定 IP | NGINX `deny` |
| **動態黑名單** | 自動封禁 | 基於行為分析 |

## 速率限制監控

| 監控項 | 說明 | 工具 |
|--------|------|------|
| 限制觸發次數 | 被限制的請求數 | Prometheus |
| 請求速率分佈 | 請求速率統計 | Grafana |
| 429 錯誤率 | 速率限制錯誤率 | Prometheus |
| 突發請求模式 | 突發請求分析 | Grafana |

## 常見問題處理

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| **誤限制** | 限制太嚴格 | 調整 per_second |
| **未限制** | 配置未生效 | 檢查中間件順序 |
| **突發被拒絕** | burst_size 太小 | 增加 burst_size |
| **內部服務被限** | 未設置白名單 | 添加白名單 |

## 速率限制測試

| 測試項目 | 方法 | 預期結果 |
|----------|------|----------|
| 正常請求 | 低於限制速率 | 200 OK |
| 超過限制 | 超過限制速率 | 429 Too Many Requests |
| 突發請求 | 短時間大量請求 | 部分成功，部分 429 |
| 恢復測試 | 等待後重新請求 | 恢復正常 |

## 程式碼範例

### Tower Governor 實作
```rust
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

pub fn rate_limit_layer() -> GovernorLayer {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(100)
            .burst_size(200)
            .finish()
            .unwrap(),
    );
    GovernorLayer::new(config)
}

// 應用中間件
.layer(rate_limit::rate_limit_layer())
```

### NGINX 配置
```nginx
# 定義限制區域
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=100r/s;

server {
    location / {
        limit_req zone=api_limit burst=200 nodelay;
        proxy_pass http://api_backend;
    }
}
```

## 進階配置

| 配置 | 說明 | 範例 |
|------|------|------|
| **多層限制** | 不同端點不同限制 | 根據路徑設定 |
| **動態調整** | 根據負載調整 | 自動擴展 |
| **地理位置限制** | 基於地理位置 | GeoIP 模組 |
| **用戶等級限制** | 不同用戶不同限制 | VIP 用戶更高限制 |

