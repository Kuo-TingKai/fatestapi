# 連線池優化 Cheat Sheet

## 連線池概念

| 概念 | 說明 | 優勢 |
|------|------|------|
| **連線池** | 預先建立的連線集合 | 避免重複建立連線開銷 |
| **連線重用** | 多個請求共享連線 | 減少 TCP 握手時間 |
| **連線管理** | 自動管理連線生命週期 | 防止連線洩漏 |
| **負載平衡** | 分散連線到多個資料庫 | 提高並發處理能力 |

## PostgreSQL 連線池 (SQLx)

| 配置項 | 說明 | 預設值 | 建議值 |
|--------|------|--------|--------|
| `max_connections` | 最大連線數 | 10 | 20-100 |
| `min_connections` | 最小連線數 | 0 | 5-10 |
| `acquire_timeout` | 獲取連線超時 | 30s | 10-30s |
| `idle_timeout` | 空閒連線超時 | 10min | 5-10min |
| `max_lifetime` | 連線最大生命週期 | 30min | 30-60min |

## SQLx 連線池實作

| 操作 | 方法 | 說明 | 範例 |
|------|------|------|------|
| 建立連線池 | `PgPool::connect()` | 建立 PostgreSQL 連線池 | `PgPool::connect(url).await?` |
| 執行查詢 | `pool.execute()` | 執行 SQL 語句 | `sqlx::query!("...").execute(pool).await?` |
| 查詢單筆 | `pool.fetch_one()` | 查詢單一記錄 | `query.fetch_one(pool).await?` |
| 查詢多筆 | `pool.fetch_all()` | 查詢多筆記錄 | `query.fetch_all(pool).await?` |
| 可選查詢 | `pool.fetch_optional()` | 查詢可能不存在的記錄 | `query.fetch_optional(pool).await?` |

## 連線池配置

| 配置方式 | 方法 | 範例 |
|----------|------|------|
| 環境變數 | `DATABASE_URL` | `postgresql://user:pass@host:5432/db` |
| 程式碼配置 | `PgPoolOptions` | `PgPoolOptions::new().max_connections(20)` |
| 連接字串參數 | URL 參數 | `?max_connections=20&connect_timeout=10` |

## 連線池最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **連線數計算** | `(核心數 * 2) + 有效磁碟數` | 通常 20-100 |
| **最小連線數** | 保持最小連線數 | 5-10 個 |
| **連線超時** | 設定合理的超時時間 | 10-30 秒 |
| **連線生命週期** | 定期回收舊連線 | 30-60 分鐘 |
| **監控連線數** | 追蹤連線使用情況 | Prometheus 指標 |

## 連線池監控指標

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 活躍連線數 | 當前使用的連線 | < 80% max |
| 空閒連線數 | 可用連線數 | > 20% max |
| 等待連線數 | 等待獲取連線的請求 | 0 |
| 連線獲取時間 | 獲取連線的延遲 | < 10ms |
| 連線錯誤率 | 連線失敗比例 | < 0.1% |

## 常見連線池問題

| 問題 | 症狀 | 解決方案 |
|------|------|----------|
| **連線洩漏** | 連線數持續增長 | 確保正確釋放連線 |
| **連線耗盡** | 無法獲取新連線 | 增加 max_connections |
| **連線超時** | 獲取連線超時 | 增加 acquire_timeout |
| **連線過多** | 資料庫負載過高 | 減少 max_connections |
| **連線過少** | 請求等待時間長 | 增加 min_connections |

## Redis 連線管理

| 配置 | 說明 | 範例 |
|------|------|------|
| 連線池 | 重用 Redis 連線 | `Arc<Mutex<redis::Client>>` |
| 異步連線 | 非阻塞連線 | `client.get_async_connection().await?` |
| 連線超時 | 連線建立超時 | `Client::open(url)?` |
| 自動重連 | 斷線自動重連 | Redis 客戶端自動處理 |

## 連線池模式

| 模式 | 說明 | 使用場景 |
|------|------|----------|
| **靜態池** | 固定大小的連線池 | 預測性負載 |
| **動態池** | 根據需求調整大小 | 變化性負載 |
| **分片池** | 多個連線池分片 | 高並發場景 |

## 程式碼範例

### 建立連線池
```rust
// 基本建立
let pool = PgPool::connect(database_url).await?;

// 自訂配置
let pool = PgPoolOptions::new()
    .max_connections(20)
    .acquire_timeout(Duration::from_secs(10))
    .connect(database_url)
    .await?;
```

### 使用連線池
```rust
// 執行查詢
let user = sqlx::query_as!(
    UserRow,
    "SELECT * FROM users WHERE id = $1",
    user_id
)
.fetch_optional(&pool)
.await?;
```

### 連線池狀態檢查
```rust
// 檢查連線池健康狀態
pool.acquire().await?; // 測試獲取連線
```

## 效能優化技巧

| 技巧 | 說明 | 效果 |
|------|------|------|
| 預先建立連線 | 啟動時建立最小連線數 | 減少首次請求延遲 |
| 連線復用 | 重用現有連線 | 減少 TCP 握手 |
| 連線超時設定 | 避免長時間等待 | 快速失敗 |
| 連線生命週期 | 定期回收舊連線 | 防止連線老化 |
| 監控和告警 | 追蹤連線使用情況 | 及時發現問題 |

## 資料庫連線字串格式

| 參數 | 說明 | 範例 |
|------|------|------|
| `host` | 資料庫主機 | `localhost` |
| `port` | 資料庫端口 | `5432` |
| `user` | 使用者名稱 | `postgres` |
| `password` | 密碼 | `postgres` |
| `dbname` | 資料庫名稱 | `fastestapi` |
| `sslmode` | SSL 模式 | `prefer` |
| `connect_timeout` | 連線超時 | `10` |

## 連線池 vs 直接連線

| 特性 | 連線池 | 直接連線 |
|------|--------|----------|
| 建立開銷 | 一次建立，多次使用 | 每次建立 |
| 連線數 | 有限制 | 無限制 |
| 管理 | 自動管理 | 手動管理 |
| 適用場景 | 高並發、頻繁查詢 | 單次查詢、低頻 |

## 故障排除

| 問題 | 檢查項目 | 解決方法 |
|------|----------|----------|
| 無法建立連線 | 資料庫是否運行 | `docker-compose ps postgres` |
| 連線超時 | 網路連線 | 檢查防火牆和網路 |
| 連線數不足 | 當前連線數 | 增加 max_connections |
| 連線洩漏 | 連線使用情況 | 檢查程式碼是否正確釋放 |

