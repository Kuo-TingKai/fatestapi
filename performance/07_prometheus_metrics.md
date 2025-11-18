# Prometheus 指標收集 Cheat Sheet

## Prometheus 概念

| 概念 | 說明 | 用途 |
|------|------|------|
| **指標 (Metric)** | 可測量的數值 | 監控系統狀態 |
| **標籤 (Label)** | 指標的維度 | 分類和過濾 |
| **時間序列** | 時間序列資料 | 歷史趨勢分析 |
| **抓取 (Scrape)** | 收集指標 | 定期拉取指標 |

## 指標類型

| 類型 | 說明 | 使用場景 |
|------|------|----------|
| **Counter** | 只增不減的計數器 | 請求總數、錯誤總數 |
| **Gauge** | 可增可減的數值 | 當前連線數、記憶體使用 |
| **Histogram** | 分佈統計 | 請求延遲分佈 |
| **Summary** | 摘要統計 | 請求延遲摘要 |

## Prometheus Rust 庫

| 操作 | 方法 | 範例 |
|------|------|------|
| 註冊 Counter | `register_counter!()` | `register_counter!("http_requests_total")` |
| 註冊 Gauge | `register_gauge!()` | `register_gauge!("active_connections")` |
| 註冊 Histogram | `register_histogram!()` | `register_histogram!("request_duration_seconds")` |
| 註冊 IntCounter | `register_int_counter!()` | `register_int_counter!("cache_hits_total")` |
| 收集指標 | `prometheus::gather()` | `prometheus::gather()` |

## 指標命名規範

| 規範 | 說明 | 範例 |
|------|------|------|
| **命名格式** | `snake_case` | `http_requests_total` |
| **單位後綴** | 包含單位 | `request_duration_seconds` |
| **類型後綴** | 包含類型 | `_total`, `_count`, `_sum` |
| **描述性** | 清楚描述用途 | `cache_hits_total` |

## Counter 指標

| 操作 | 方法 | 說明 |
|------|------|------|
| 增加 | `.inc()` | 增加 1 |
| 增加指定值 | `.inc_by(n)` | 增加 n |
| 獲取值 | `.get()` | 獲取當前值 |
| 重置 | `.reset()` | 重置為 0 |

## Gauge 指標

| 操作 | 方法 | 說明 |
|------|------|------|
| 設定值 | `.set(value)` | 設定為指定值 |
| 增加 | `.inc()` | 增加 1 |
| 減少 | `.dec()` | 減少 1 |
| 增加指定值 | `.inc_by(n)` | 增加 n |
| 減少指定值 | `.dec_by(n)` | 減少 n |

## Histogram 指標

| 操作 | 方法 | 說明 |
|------|------|------|
| 觀察值 | `.observe(value)` | 記錄一個觀察值 |
| 獲取分佈 | `.get()` | 獲取分佈統計 |

## 指標實作範例

| 指標 | 類型 | 實作 |
|------|------|------|
| 請求總數 | Counter | `register_counter!("http_requests_total")` |
| 快取命中 | IntCounter | `register_int_counter!("cache_hits_total")` |
| 請求延遲 | Histogram | `register_histogram!("request_duration_seconds")` |
| 活躍連線 | Gauge | `register_gauge!("active_connections")` |

## 指標標籤

| 概念 | 說明 | 範例 |
|------|------|------|
| **標籤鍵值對** | 分類指標 | `{method="GET", status="200"}` |
| **動態標籤** | 根據上下文設定 | `with_label_values(&["GET", "200"])` |
| **標籤組合** | 多個標籤組合 | `{endpoint="/api/users", method="GET"}` |

## 指標端點

| 端點 | 說明 | 格式 |
|------|------|------|
| `/metrics` | Prometheus 指標 | Prometheus 文字格式 |
| 內容類型 | `text/plain` | `Content-Type: text/plain` |

## 指標查詢 (PromQL)

| 查詢 | 說明 | 範例 |
|------|------|------|
| 選擇指標 | 選擇特定指標 | `http_requests_total` |
| 過濾標籤 | 根據標籤過濾 | `http_requests_total{method="GET"}` |
| 計算速率 | 計算變化速率 | `rate(http_requests_total[5m])` |
| 計算增量 | 計算變化量 | `increase(http_requests_total[1h])` |
| 聚合函數 | 聚合多個指標 | `sum(http_requests_total)` |

## 常用 PromQL 查詢

| 查詢 | 說明 | 用途 |
|------|------|------|
| `rate(metric[5m])` | 每秒速率 | 請求速率 |
| `increase(metric[1h])` | 增量 | 總請求數 |
| `sum(metric)` | 總和 | 聚合指標 |
| `avg(metric)` | 平均值 | 平均延遲 |
| `histogram_quantile(0.95, metric)` | 分位數 | P95 延遲 |

## 程式碼範例

### 註冊指標
```rust
use prometheus::{register_counter, register_histogram, register_int_counter};

let requests_total = register_counter!(
    "http_requests_total",
    "Total number of HTTP requests"
).unwrap();

let cache_hits = register_int_counter!(
    "cache_hits_total",
    "Total number of cache hits"
).unwrap();

let request_duration = register_histogram!(
    "http_request_duration_seconds",
    "HTTP request duration in seconds"
).unwrap();
```

### 記錄指標
```rust
// 增加計數器
requests_total.inc();

// 記錄直方圖
request_duration.observe(0.05); // 50ms

// 記錄快取命中
cache_hits.inc();
```

### 指標端點
```rust
use axum::response::Response;
use prometheus::{Encoder, TextEncoder};

pub async fn metrics_handler() -> Result<Response<String>, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    
    encoder.encode(&metric_families, &mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let body = String::from_utf8(buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", encoder.format_type())
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}
```

## 指標最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **命名規範** | 使用標準命名 | `snake_case`，包含單位 |
| **標籤數量** | 限制標籤數量 | < 10 個標籤 |
| **標籤基數** | 避免高基數標籤 | 避免用戶 ID 等 |
| **指標粒度** | 適當的指標粒度 | 平衡詳細度和效能 |
| **指標清理** | 定期清理未使用指標 | 避免指標爆炸 |

## 監控指標建議

| 類別 | 指標 | 類型 |
|------|------|------|
| **HTTP** | `http_requests_total` | Counter |
| **HTTP** | `http_request_duration_seconds` | Histogram |
| **快取** | `cache_hits_total` | Counter |
| **快取** | `cache_misses_total` | Counter |
| **資料庫** | `db_queries_total` | Counter |
| **資料庫** | `db_query_duration_seconds` | Histogram |
| **連線** | `active_connections` | Gauge |

## Prometheus 配置

| 配置項 | 說明 | 範例 |
|--------|------|------|
| `scrape_interval` | 抓取間隔 | `15s` |
| `scrape_timeout` | 抓取超時 | `10s` |
| `metrics_path` | 指標路徑 | `/metrics` |
| `targets` | 目標列表 | `['localhost:3000']` |

## Grafana 儀表板

| 面板類型 | 說明 | 用途 |
|----------|------|------|
| **Graph** | 時間序列圖表 | 趨勢分析 |
| **Stat** | 單一數值 | 當前狀態 |
| **Table** | 表格 | 詳細資料 |
| **Heatmap** | 熱力圖 | 分佈分析 |

## 常見指標查詢

| 查詢 | PromQL | 說明 |
|------|--------|------|
| 請求速率 | `rate(http_requests_total[5m])` | 每秒請求數 |
| 錯誤率 | `rate(http_requests_total{status="500"}[5m]) / rate(http_requests_total[5m])` | 錯誤比例 |
| P95 延遲 | `histogram_quantile(0.95, http_request_duration_seconds_bucket)` | 95 分位數延遲 |
| 快取命中率 | `cache_hits_total / (cache_hits_total + cache_misses_total)` | 命中率 |

## 告警規則

| 規則 | PromQL | 說明 |
|------|--------|------|
| 高錯誤率 | `rate(http_requests_total{status="500"}[5m]) > 0.01` | 錯誤率 > 1% |
| 高延遲 | `histogram_quantile(0.95, http_request_duration_seconds_bucket) > 0.1` | P95 > 100ms |
| 低快取命中率 | `cache_hits_total / (cache_hits_total + cache_misses_total) < 0.8` | 命中率 < 80% |

## 效能考量

| 考量 | 說明 | 建議 |
|------|------|------|
| **指標數量** | 限制指標總數 | < 1000 個 |
| **標籤基數** | 避免高基數 | 標籤值 < 100 |
| **更新頻率** | 指標更新頻率 | 避免過於頻繁 |
| **儲存成本** | 時間序列儲存 | 設定保留時間 |

## 故障排除

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| **指標未顯示** | 端點未正確配置 | 檢查 `/metrics` 端點 |
| **指標過多** | 標籤基數過高 | 減少標籤或合併指標 |
| **查詢慢** | 指標數量過多 | 優化查詢或減少指標 |
| **記憶體使用高** | 指標緩衝區過大 | 限制指標數量 |

