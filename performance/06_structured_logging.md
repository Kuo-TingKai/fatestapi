# 結構化日誌 (JSON 格式) Cheat Sheet

## 結構化日誌概念

| 概念 | 說明 | 優勢 |
|------|------|------|
| **結構化日誌** | JSON 格式日誌 | 易於解析和查詢 |
| **日誌級別** | 錯誤、警告、資訊等 | 過濾和分類 |
| **日誌上下文** | 請求 ID、用戶 ID 等 | 追蹤請求流程 |
| **分散式追蹤** | 跨服務追蹤 | 追蹤完整請求鏈 |

## 日誌級別

| 級別 | 說明 | 使用場景 |
|------|------|----------|
| **ERROR** | 錯誤 | 系統錯誤、異常 |
| **WARN** | 警告 | 潛在問題、降級 |
| **INFO** | 資訊 | 一般資訊、請求處理 |
| **DEBUG** | 除錯 | 詳細除錯資訊 |
| **TRACE** | 追蹤 | 最詳細的追蹤資訊 |

## Tracing 庫配置

| 配置項 | 說明 | 範例 |
|--------|------|------|
| `tracing_subscriber` | 日誌訂閱者 | `tracing_subscriber::fmt()` |
| `.with_env_filter()` | 環境變數過濾 | `.with_env_filter(EnvFilter::from_env("RUST_LOG"))` |
| `.json()` | JSON 格式輸出 | `.json()` |
| `.init()` | 初始化 | `.init()` |

## Tracing 實作

| 操作 | 方法 | 範例 |
|------|------|------|
| 資訊日誌 | `info!()` | `info!("Server starting on {}", addr);` |
| 錯誤日誌 | `error!()` | `error!("Failed to connect: {}", e);` |
| 警告日誌 | `warn!()` | `warn!("Cache miss for key: {}", key);` |
| 除錯日誌 | `debug!()` | `debug!("Processing request: {:?}", req);` |
| 追蹤日誌 | `trace!()` | `trace!("Entering function");` |

## 屬性宏

| 宏 | 說明 | 範例 |
|----|------|------|
| `#[instrument]` | 自動追蹤函數 | `#[instrument] async fn handler() {}` |
| `#[instrument(skip(...))]` | 跳過參數追蹤 | `#[instrument(skip(state))]` |
| `#[instrument(fields(...))]` | 自訂欄位 | `#[instrument(fields(user_id = %id))]` |

## 日誌格式配置

| 格式 | 說明 | 配置 |
|------|------|------|
| **JSON** | JSON 格式 | `.json()` |
| **Pretty** | 可讀格式 | `.pretty()` |
| **Compact** | 緊湊格式 | `.compact()` |
| **Full** | 完整格式 | `.full()` |

## 環境變數過濾

| 變數 | 說明 | 範例 |
|------|------|------|
| `RUST_LOG` | 日誌級別過濾 | `fastestapi=info,tower_http=info` |
| 格式 | `module=level` | `fastestapi=debug` |
| 多模組 | 逗號分隔 | `fastestapi=info,db=debug` |

## 日誌欄位

| 欄位 | 說明 | 範例 |
|------|------|------|
| `timestamp` | 時間戳 | `2024-01-01T00:00:00Z` |
| `level` | 日誌級別 | `INFO`, `ERROR` |
| `target` | 目標模組 | `fastestapi::handler` |
| `message` | 日誌訊息 | `Server starting` |
| `fields` | 自訂欄位 | `{"user_id": "123"}` |

## 結構化日誌範例

| 類型 | 範例 | JSON 輸出 |
|------|------|-----------|
| 簡單日誌 | `info!("User created");` | `{"level":"INFO","message":"User created"}` |
| 帶參數 | `info!(user_id = %id, "User created");` | `{"level":"INFO","user_id":"123","message":"User created"}` |
| 錯誤日誌 | `error!(error = ?e, "Failed");` | `{"level":"ERROR","error":"...","message":"Failed"}` |

## 日誌最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **適當級別** | 使用正確的日誌級別 | ERROR 用於錯誤，INFO 用於資訊 |
| **結構化資料** | 使用欄位而非字串拼接 | `user_id = %id` 而非 `format!("user_id: {}", id)` |
| **避免敏感資訊** | 不記錄密碼、token | 使用 `skip` 或過濾 |
| **上下文資訊** | 包含請求 ID、用戶 ID | 使用 `#[instrument]` |
| **效能考量** | 避免高頻日誌 | DEBUG/TRACE 僅開發環境 |

## 日誌輸出目標

| 目標 | 說明 | 配置 |
|------|------|------|
| **標準輸出** | stdout | 預設 |
| **標準錯誤** | stderr | `.with_writer(std::io::stderr)` |
| **檔案** | 寫入檔案 | `.with_writer(file)` |
| **系統日誌** | syslog | 使用 syslog 後端 |

## 分散式追蹤

| 概念 | 說明 | 實作 |
|------|------|------|
| **Trace ID** | 追蹤 ID | 自動生成 |
| **Span** | 追蹤區間 | `#[instrument]` |
| **Parent Span** | 父區間 | 自動關聯 |
| **Context** | 追蹤上下文 | 自動傳播 |

## 日誌聚合

| 工具 | 說明 | 用途 |
|------|------|------|
| **ELK Stack** | Elasticsearch, Logstash, Kibana | 日誌聚合和分析 |
| **Loki** | Grafana Loki | 日誌聚合 |
| **Fluentd** | 日誌收集器 | 日誌轉發 |
| **Prometheus** | 指標收集 | 日誌指標化 |

## 程式碼範例

### 基本配置
```rust
use tracing_subscriber::{fmt, EnvFilter};

tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "fastestapi=info".into()),
    )
    .json()
    .init();
```

### 日誌記錄
```rust
use tracing::{info, error, warn, debug};

// 資訊日誌
info!("Server starting on {}", addr);

// 錯誤日誌
error!(error = ?e, "Failed to connect to database");

// 警告日誌
warn!(cache_key = %key, "Cache miss");

// 除錯日誌
debug!(user_id = %id, "Processing user request");
```

### 函數追蹤
```rust
#[instrument]
async fn get_user(user_id: Uuid) -> Result<User> {
    // 自動記錄函數進入/退出和參數
    let user = db.get_user(user_id).await?;
    Ok(user)
}

#[instrument(skip(state))]
async fn handler(State(state): State<AppState>) -> Result<Response> {
    // 跳過 state 參數的追蹤
    // ...
}
```

### 自訂欄位
```rust
#[instrument(fields(user_id = %id, endpoint = "get_user"))]
async fn get_user(id: Uuid) -> Result<User> {
    // 自訂追蹤欄位
    // ...
}
```

## 日誌查詢

| 查詢類型 | 說明 | 範例 |
|----------|------|------|
| **級別過濾** | 過濾特定級別 | `level=ERROR` |
| **模組過濾** | 過濾特定模組 | `target=fastestapi::db` |
| **欄位查詢** | 查詢特定欄位 | `user_id=123` |
| **時間範圍** | 時間範圍查詢 | `timestamp>2024-01-01` |

## 效能考量

| 考量 | 說明 | 建議 |
|------|------|------|
| **日誌開銷** | 日誌記錄有成本 | 生產環境使用 INFO 以上 |
| **序列化成本** | JSON 序列化開銷 | 避免高頻日誌 |
| **I/O 開銷** | 寫入日誌的 I/O | 使用異步寫入 |
| **記憶體使用** | 日誌緩衝區 | 限制緩衝區大小 |

## 常見問題

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| **日誌過多** | 級別設定太低 | 提高日誌級別 |
| **日誌遺失** | 緩衝區滿 | 增加緩衝區或使用異步 |
| **格式不一致** | 未使用結構化 | 使用 JSON 格式 |
| **敏感資訊洩露** | 記錄敏感資料 | 使用 `skip` 或過濾 |

## 日誌監控

| 監控項 | 說明 | 工具 |
|--------|------|------|
| **錯誤率** | ERROR 日誌比例 | Prometheus |
| **日誌量** | 日誌產生速率 | Prometheus |
| **日誌大小** | 日誌檔案大小 | 檔案系統監控 |
| **日誌延遲** | 日誌寫入延遲 | 自訂指標 |

