# 快速開始指南

## 前置需求

- Docker 和 Docker Compose
- Rust 1.75+ (可選，如果要在本地編譯)

## 步驟 1: 啟動基礎服務

```bash
# 使用提供的腳本
./scripts/start.sh

# 或手動啟動
docker-compose up -d postgres redis prometheus grafana nginx
```

## 步驟 2: 等待服務就緒

等待約 10-15 秒讓所有服務啟動完成。

```bash
# 檢查服務狀態
docker-compose ps
```

## 步驟 3: 執行 API 服務

### 方式 A: 使用 Cargo (推薦用於開發)

```bash
# 設定環境變數
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/fastestapi"
export REDIS_URL="redis://localhost:6379"
export PORT=3000

# 編譯並執行
cargo run --release
```

### 方式 B: 使用 Docker

```bash
# 建置映像檔
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

## 步驟 4: 測試 API

```bash
# 使用提供的測試腳本
./scripts/test-api.sh

# 或手動測試
curl http://localhost:3000/health
```

## 步驟 5: 查看監控

- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3001 (admin/admin)
- **API 指標**: http://localhost:3000/metrics

## 效能測試

### 使用 wrk

```bash
# 安裝 wrk (macOS)
brew install wrk

# 執行壓力測試
wrk -t12 -c400 -d30s http://localhost:3000/api/users
```

### 使用 Apache Bench

```bash
# 安裝 ab (macOS)
brew install httpd

# 執行壓力測試
ab -n 10000 -c 100 http://localhost:3000/api/users
```

## 常見問題

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

# 測試連線
docker exec -it fastestapi_redis redis-cli ping
```

### 編譯錯誤

```bash
# 清理並重新編譯
cargo clean
cargo build --release
```

## 下一步

- 閱讀 [README.md](README.md) 了解完整功能
- 查看 API 端點文檔
- 自訂 NGINX 配置
- 設定 Grafana 儀表板

