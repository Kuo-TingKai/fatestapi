# FastestAPI - 超高效能 API 服務

這是一個使用最先進開源技術建構的高效能 API 服務，能夠應付超大流量並提供超快反應速度。

## 技術棧

### 核心技術
- **Rust + Axum**: 零成本抽象的現代 Web 框架，提供極致的效能
- **PostgreSQL**: 強大的關聯式資料庫，支援讀寫分離和索引優化
- **Redis**: 高效能記憶體快取，減少資料庫壓力
- **NGINX**: 反向代理和負載平衡，提供 HTTP/2 支援和壓縮
- **Prometheus + Grafana**: 完整的監控和可觀測性解決方案

### 效能優化特性
- ✅ 多層快取策略（應用層 + Redis）
- ✅ 連線池優化
- ✅ HTTP/2 和壓縮支援
- ✅ 速率限制（Rate Limiting）
- ✅ 非同步 I/O（Tokio）
- ✅ 結構化日誌（JSON 格式）
- ✅ Prometheus 指標收集
- ✅ NGINX 反向代理快取

## 專案結構

```
fastestapi/
├── src/
│   ├── main.rs          # 主應用程式
│   ├── config.rs        # 配置管理
│   ├── db.rs            # 資料庫操作
│   ├── cache.rs         # Redis 快取層
│   ├── metrics.rs       # Prometheus 指標
│   ├── rate_limit.rs    # 速率限制
│   └── error.rs         # 錯誤處理
├── migrations/          # 資料庫遷移
├── nginx/              # NGINX 配置
├── prometheus/         # Prometheus 配置
├── grafana/            # Grafana 配置
├── docker-compose.yml  # Docker Compose 配置
├── Dockerfile          # Docker 映像檔
└── Cargo.toml          # Rust 依賴
```

## 快速開始

### 前置需求
- Docker 和 Docker Compose
- Rust 1.75+ (如果要在本地編譯)

### 1. 啟動基礎服務

```bash
# 啟動 PostgreSQL, Redis, Prometheus, Grafana, NGINX
docker-compose up -d postgres redis prometheus grafana nginx
```

### 2. 等待服務就緒

```bash
# 檢查服務狀態
docker-compose ps
```

### 3. 編譯並執行 API 服務

#### 方式 A: 使用 Cargo (本地執行)

```bash
# 安裝依賴並編譯
cargo build --release

# 設定環境變數
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/fastestapi"
export REDIS_URL="redis://localhost:6379"
export PORT=3000

# 執行
cargo run --release
```

#### 方式 B: 使用 Docker

```bash
# 建置 Docker 映像檔
docker build -t fastestapi .

# 執行容器
docker run -d \
  --name fastestapi \
  --network fastestapi_fastestapi_network \
  -p 3000:3000 \
  -e DATABASE_URL="postgresql://postgres:postgres@postgres:5432/fastestapi" \
  -e REDIS_URL="redis://redis:6379" \
  fastestapi
```

### 4. 驗證服務

```bash
# 健康檢查
curl http://localhost/health

# 或直接訪問 API
curl http://localhost:3000/health
```

## API 端點

### 健康檢查
```bash
GET /health
```

### 使用者管理
```bash
# 列出使用者
GET /api/users?limit=10&offset=0

# 取得單一使用者
GET /api/users/{id}

# 建立使用者
POST /api/users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com"
}
```

### 統計資訊
```bash
GET /api/stats
```

### Prometheus 指標
```bash
GET /metrics
```

## 效能測試

### 使用 Apache Bench (ab)

```bash
# 安裝 ab (macOS)
brew install httpd

# 執行壓力測試
ab -n 10000 -c 100 http://localhost/api/users
```

### 使用 wrk

```bash
# 安裝 wrk
brew install wrk

# 執行壓力測試
wrk -t12 -c400 -d30s http://localhost/api/users
```

### 使用 k6

```bash
# 安裝 k6
brew install k6

# 建立測試腳本 test.js
cat > test.js << EOF
import http from 'k6/http';
import { check } from 'k6';

export let options = {
  stages: [
    { duration: '30s', target: 100 },
    { duration: '1m', target: 500 },
    { duration: '30s', target: 1000 },
  ],
};

export default function () {
  let res = http.get('http://localhost/api/users');
  check(res, { 'status was 200': (r) => r.status == 200 });
}
EOF

# 執行測試
k6 run test.js
```

## 監控和可觀測性

### Prometheus
- 訪問: http://localhost:9090
- 查詢範例:
  - `rate(http_requests_total[5m])` - 請求速率
  - `cache_hits_total / (cache_hits_total + cache_misses_total)` - 快取命中率

### Grafana
- 訪問: http://localhost:3001
- 預設帳號: `admin` / `admin`
- 資料來源已自動配置為 Prometheus

## 環境變數

| 變數 | 說明 | 預設值 |
|------|------|--------|
| `PORT` | API 服務埠號 | `3000` |
| `DATABASE_URL` | PostgreSQL 連線字串 | `postgresql://postgres:postgres@localhost:5432/fastestapi` |
| `REDIS_URL` | Redis 連線字串 | `redis://localhost:6379` |

## 效能優化建議

### 1. 資料庫優化
- 使用連線池（已實作）
- 適當的索引（已建立）
- 考慮讀寫分離（生產環境）

### 2. 快取策略
- 熱點資料快取（已實作）
- 快取失效策略（已實作）
- 考慮使用 Redis Cluster（大規模部署）

### 3. NGINX 優化
- 調整 `worker_processes` 和 `worker_connections`
- 啟用 HTTP/2（需要 SSL 憑證）
- 調整快取策略

### 4. 應用層優化
- 使用 `--release` 模式編譯
- 調整 Tokio 執行緒池大小
- 使用 `jemalloc` 記憶體分配器

## 擴展性

### 水平擴展
1. 使用 Kubernetes 部署多個 API 實例
2. NGINX 自動負載平衡
3. Redis Cluster 分散式快取
4. PostgreSQL 讀寫分離

### 垂直擴展
1. 增加伺服器資源（CPU、記憶體）
2. 調整連線池大小
3. 增加 NGINX worker 數量

## 故障排除

### 資料庫連線失敗
```bash
# 檢查 PostgreSQL 是否運行
docker-compose ps postgres

# 查看日誌
docker-compose logs postgres
```

### Redis 連線失敗
```bash
# 檢查 Redis 是否運行
docker-compose ps redis

# 測試 Redis 連線
docker exec -it fastestapi_redis redis-cli ping
```

### 編譯錯誤
```bash
# 清理並重新編譯
cargo clean
cargo build --release
```

## 授權

MIT License

## 貢獻

歡迎提交 Issue 和 Pull Request！

