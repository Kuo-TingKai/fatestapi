# FastestAPI 核心技術 Cheat Sheet

## 技術棧總覽

| 類別 | 技術 | 版本 | 用途 | 官方文檔 |
|------|------|------|------|----------|
| **語言** | Rust | 1.75+ | 核心開發語言 | https://www.rust-lang.org/ |
| **Web 框架** | Axum | 0.7 | 高效能 HTTP 框架 | https://docs.rs/axum/ |
| **異步運行時** | Tokio | 1.x | 異步 I/O 和任務調度 | https://tokio.rs/ |
| **資料庫** | PostgreSQL | 16 | 主要資料庫 | https://www.postgresql.org/ |
| **資料庫驅動** | SQLx | 0.7 | 異步 PostgreSQL 驅動 | https://docs.rs/sqlx/ |
| **快取** | Redis | 7 | 分散式快取 | https://redis.io/ |
| **Redis 客戶端** | redis-rs | 0.24 | Rust Redis 客戶端 | https://docs.rs/redis/ |
| **反向代理** | NGINX | latest | 負載平衡和快取 | https://nginx.org/ |
| **監控** | Prometheus | latest | 指標收集 | https://prometheus.io/ |
| **視覺化** | Grafana | latest | 監控儀表板 | https://grafana.com/ |

## Rust 核心依賴

| 套件 | 版本 | 功能 | 關鍵特性 |
|------|------|------|----------|
| `axum` | 0.7 | Web 框架 | 類型安全路由、中間件系統 |
| `tokio` | 1.x | 異步運行時 | 異步 I/O、任務調度、定時器 |
| `sqlx` | 0.7 | 資料庫工具 | 編譯時查詢檢查、連接池 |
| `redis` | 0.24 | Redis 客戶端 | 異步操作、連接管理 |
| `tower` | 0.4 | 中間件框架 | 可組合的中間件 |
| `tower-http` | 0.5 | HTTP 中間件 | 壓縮、CORS、追蹤 |
| `tower_governor` | 0.2 | 速率限制 | 基於 IP 的速率限制 |
| `prometheus` | 0.13 | 指標庫 | Prometheus 指標格式 |
| `serde` | 1.0 | 序列化 | JSON 序列化/反序列化 |
| `tracing` | 0.1 | 日誌追蹤 | 結構化日誌、分散式追蹤 |

## 資料庫操作

| 操作 | SQLx 方法 | 範例 |
|------|-----------|------|
| 查詢單筆 | `query_as!` | `sqlx::query_as!(UserRow, "SELECT * FROM users WHERE id = $1", id)` |
| 查詢多筆 | `query_as!` + `fetch_all` | `sqlx::query_as!(UserRow, "SELECT * FROM users").fetch_all(pool).await?` |
| 執行更新 | `query!` + `execute` | `sqlx::query!("INSERT INTO users ...").execute(pool).await?` |
| 查詢標量 | `query_scalar!` | `sqlx::query_scalar!("SELECT COUNT(*) FROM users").fetch_one(pool).await?` |
| 遷移 | `sqlx::migrate!` | `sqlx::migrate!("migrations").run(pool).await?` |

## Redis 快取操作

| 操作 | 方法 | TTL 設定 |
|------|------|----------|
| 取得快取 | `cache.get::<T>(key)` | - |
| 設定快取 | `cache.set(key, value, ttl_seconds)` | 秒數 |
| 刪除快取 | `cache.delete(key)` | - |
| 快取鍵命名 | `format!("user:{}", user_id)` | - |
| 列表快取鍵 | `format!("users:limit:{}:offset:{}", limit, offset)` | - |

## Axum 路由定義

| 路由類型 | 語法 | 範例 |
|----------|------|------|
| GET | `get(handler)` | `.route("/users", get(list_users))` |
| POST | `post(handler)` | `.route("/users", post(create_user))` |
| PUT | `put(handler)` | `.route("/users/:id", put(update_user))` |
| DELETE | `delete(handler)` | `.route("/users/:id", delete(delete_user))` |
| 路徑參數 | `Path(id): Path<Uuid>` | `async fn get_user(Path(id): Path<Uuid>)` |
| 查詢參數 | `Query(params): Query<HashMap>` | `Query(params): Query<HashMap<String, String>>` |
| JSON 請求體 | `Json(payload): Json<Request>` | `Json(payload): Json<CreateUserRequest>` |
| 狀態注入 | `State(state): State<AppState>` | `State(state): State<AppState>` |

## 中間件配置

| 中間件 | 功能 | 配置方式 |
|--------|------|----------|
| `TraceLayer` | 請求追蹤 | `.layer(TraceLayer::new_for_http())` |
| `CompressionLayer` | Gzip 壓縮 | `.layer(CompressionLayer::new())` |
| `CorsLayer` | CORS 支援 | `.layer(CorsLayer::permissive())` |
| `GovernorLayer` | 速率限制 | `.layer(rate_limit_layer())` |

## 速率限制配置

| 參數 | 預設值 | 說明 |
|------|--------|------|
| `per_second` | 100 | 每秒允許的請求數 |
| `burst_size` | 200 | 突發請求允許數量 |
| 限制範圍 | IP 地址 | 基於客戶端 IP |

## Prometheus 指標

| 指標類型 | 註冊方法 | 範例 |
|----------|----------|------|
| Counter | `register_counter!` | `register_counter!("http_requests_total")` |
| IntCounter | `register_int_counter!` | `register_int_counter!("cache_hits_total")` |
| Histogram | `register_histogram!` | `register_histogram!("request_duration_seconds")` |
| Gauge | `register_gauge!` | `register_gauge!("active_connections")` |

## NGINX 配置要點

| 配置項 | 位置 | 說明 |
|--------|------|------|
| `worker_processes` | `nginx.conf` | Worker 進程數（auto = CPU 核心數） |
| `worker_connections` | `events {}` | 每個 worker 最大連接數 |
| `proxy_cache_path` | `http {}` | 快取路徑配置（必須在 http 區塊） |
| `limit_req_zone` | `http {}` | 速率限制區域定義 |
| `limit_req` | `server {}` | 速率限制應用 |
| `proxy_cache` | `location {}` | 啟用快取 |
| `gzip` | `http {}` | 啟用壓縮 |

## Docker Compose 服務

| 服務 | 映像檔 | 端口 | 用途 |
|------|--------|------|------|
| `postgres` | `postgres:16-alpine` | 5432 | 資料庫 |
| `redis` | `redis:7-alpine` | 6379 | 快取 |
| `prometheus` | `prom/prometheus:latest` | 9090 | 指標收集 |
| `grafana` | `grafana/grafana:latest` | 3001 | 監控視覺化 |
| `nginx` | `nginx:alpine` | 80, 443 | 反向代理 |

## 環境變數

| 變數名 | 預設值 | 說明 |
|--------|--------|------|
| `PORT` | `3000` | API 服務端口 |
| `DATABASE_URL` | `postgresql://postgres:postgres@localhost:5432/fastestapi` | 資料庫連接字串 |
| `REDIS_URL` | `redis://localhost:6379` | Redis 連接字串 |
| `RUST_LOG` | `fastestapi=info` | 日誌級別 |

## 常用命令

| 操作 | 命令 |
|------|------|
| 啟動基礎服務 | `docker-compose up -d postgres redis prometheus grafana nginx` |
| 停止所有服務 | `docker-compose down` |
| 查看服務狀態 | `docker-compose ps` |
| 查看日誌 | `docker-compose logs [service]` |
| 編譯專案 | `cargo build --release` |
| 執行專案 | `cargo run --release` |
| 執行測試 | `cargo test` |
| 清理編譯 | `cargo clean` |
| 資料庫遷移 | `sqlx migrate run` |

## 效能優化技巧

| 優化項目 | 方法 | 效果 |
|---------|------|------|
| 連線池 | `PgPool::connect()` | 重用資料庫連接 |
| 快取策略 | Redis + TTL | 減少資料庫查詢 |
| 非同步 I/O | Tokio | 高並發處理 |
| 壓縮 | Gzip | 減少傳輸大小 |
| 索引 | 資料庫索引 | 加速查詢 |
| 連接復用 | NGINX keepalive | 減少連接開銷 |

## API 端點速查

| 端點 | 方法 | 功能 | 快取 |
|------|------|------|------|
| `/health` | GET | 健康檢查 | ❌ |
| `/api/users` | GET | 列出使用者 | ✅ (60s) |
| `/api/users` | POST | 建立使用者 | ❌ |
| `/api/users/:id` | GET | 取得使用者 | ✅ (300s) |
| `/api/stats` | GET | 統計資訊 | ❌ |
| `/metrics` | GET | Prometheus 指標 | ❌ |

## 錯誤處理

| 錯誤類型 | 處理方式 | HTTP 狀態碼 |
|----------|----------|-------------|
| `AppError::NotFound` | 資源不存在 | 404 |
| `AppError::Database` | 資料庫錯誤 | 500 |
| `AppError::Redis` | 快取錯誤 | 500 |
| `AppError::Serialization` | 序列化錯誤 | 400 |
| `AppError::Internal` | 內部錯誤 | 500 |

## 日誌級別

| 級別 | 用途 | 範例 |
|------|------|------|
| `error` | 錯誤訊息 | 資料庫連接失敗 |
| `warn` | 警告訊息 | 快取失效 |
| `info` | 一般資訊 | 服務啟動、請求處理 |
| `debug` | 除錯資訊 | 詳細的執行流程 |
| `trace` | 追蹤資訊 | 函數進入/退出 |

## 監控指標查詢 (Prometheus)

| 查詢 | 說明 |
|------|------|
| `rate(http_requests_total[5m])` | 請求速率 |
| `cache_hits_total / (cache_hits_total + cache_misses_total)` | 快取命中率 |
| `http_request_duration_seconds` | 請求延遲分佈 |
| `up{job="fastestapi"}` | 服務可用性 |

## 故障排除

| 問題 | 檢查項目 | 解決方法 |
|------|----------|----------|
| 資料庫連線失敗 | `docker-compose ps postgres` | 檢查容器狀態 |
| Redis 連線失敗 | `docker exec -it fastestapi_redis redis-cli ping` | 測試 Redis 連線 |
| 編譯錯誤 | `cargo clean && cargo build` | 清理並重新編譯 |
| NGINX 配置錯誤 | `docker-compose logs nginx` | 查看錯誤日誌 |
| 端口被佔用 | `lsof -i :3000` | 檢查端口使用情況 |

## 效能基準

| 指標 | 目標值 | 測試工具 |
|------|--------|----------|
| 延遲 (P95) | < 10ms | wrk, ab |
| 吞吐量 | > 100,000 req/s | wrk, k6 |
| 可用性 | 99.99% | Prometheus |
| 錯誤率 | < 0.01% | Prometheus |

## 參考資源

| 資源類型 | 連結 |
|---------|------|
| Rust 官方文檔 | https://doc.rust-lang.org/ |
| Axum 文檔 | https://docs.rs/axum/ |
| Tokio 指南 | https://tokio.rs/tokio/tutorial |
| SQLx 文檔 | https://docs.rs/sqlx/ |
| NGINX 文檔 | https://nginx.org/en/docs/ |
| Prometheus 文檔 | https://prometheus.io/docs/ |

