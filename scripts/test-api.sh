#!/bin/bash

# API 測試腳本

BASE_URL="${1:-http://localhost:3000}"

echo "🧪 Testing FastestAPI at $BASE_URL"
echo ""

# 健康檢查
echo "1. Health Check..."
curl -s "$BASE_URL/health" | jq '.' || echo "Failed"
echo ""

# 列出使用者
echo "2. List Users..."
curl -s "$BASE_URL/api/users?limit=5" | jq '.' || echo "Failed"
echo ""

# 建立使用者
echo "3. Create User..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "email": "test@example.com"
  }')
echo "$USER_RESPONSE" | jq '.' || echo "Failed"
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.id')
echo ""

# 取得單一使用者
if [ "$USER_ID" != "null" ] && [ -n "$USER_ID" ]; then
    echo "4. Get User by ID ($USER_ID)..."
    curl -s "$BASE_URL/api/users/$USER_ID" | jq '.' || echo "Failed"
    echo ""
fi

# 統計資訊
echo "5. Get Stats..."
curl -s "$BASE_URL/api/stats" | jq '.' || echo "Failed"
echo ""

# Prometheus 指標
echo "6. Prometheus Metrics..."
curl -s "$BASE_URL/metrics" | head -20
echo ""

echo "✅ Tests completed!"

