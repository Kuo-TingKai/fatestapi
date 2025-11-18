#!/bin/bash

# FastestAPI 啟動腳本

set -e

echo "🚀 Starting FastestAPI services..."

# 檢查 Docker 是否運行
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# 啟動基礎服務
echo "📦 Starting PostgreSQL, Redis, Prometheus, Grafana, and NGINX..."
docker-compose up -d postgres redis prometheus grafana nginx

# 等待服務就緒
echo "⏳ Waiting for services to be ready..."
sleep 5

# 檢查服務狀態
echo "🔍 Checking service status..."
docker-compose ps

echo ""
echo "✅ Services are ready!"
echo ""
echo "📊 Access points:"
echo "   - API: http://localhost:3000"
echo "   - NGINX: http://localhost"
echo "   - Prometheus: http://localhost:9090"
echo "   - Grafana: http://localhost:3001 (admin/admin)"
echo ""
echo "🔧 To run the API server:"
echo "   export DATABASE_URL='postgresql://postgres:postgres@localhost:5432/fastestapi'"
echo "   export REDIS_URL='redis://localhost:6379'"
echo "   cargo run --release"
echo ""

