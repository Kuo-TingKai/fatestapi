# 多層快取策略 Cheat Sheet

## 快取層級架構

| 層級 | 位置 | 技術 | 速度 | 容量 | 用途 |
|------|------|------|------|------|------|
| **L1** | 應用內記憶體 | Rust 記憶體 | 最快 | 最小 | 熱點資料 |
| **L2** | 分散式快取 | Redis | 快 | 中等 | 共享快取 |
| **L3** | 資料庫 | PostgreSQL | 慢 | 最大 | 持久化資料 |

## Redis 快取實作

| 操作 | 方法 | 參數 | 範例 |
|------|------|------|------|
| 初始化 | `Cache::new(redis_url)` | Redis URL | `Cache::new("redis://localhost:6379")` |
| 取得快取 | `cache.get::<T>(key)` | 鍵名、類型 | `cache.get::<User>("user:123").await?` |
| 設定快取 | `cache.set(key, value, ttl)` | 鍵、值、TTL | `cache.set("user:123", &user, 300).await?` |
| 刪除快取 | `cache.delete(key)` | 鍵名 | `cache.delete("user:123").await?` |

## 快取鍵命名規範

| 類型 | 格式 | 範例 |
|------|------|------|
| 單一資源 | `{resource}:{id}` | `user:550e8400-e29b-41d4-a716-446655440000` |
| 列表查詢 | `{resource}:limit:{limit}:offset:{offset}` | `users:limit:10:offset:0` |
| 統計資料 | `{resource}:stats:{type}` | `users:stats:count` |
| 關聯資料 | `{resource}:{id}:{related}` | `user:123:posts` |

## TTL (Time To Live) 策略

| 資料類型 | TTL | 理由 | 範例 |
|----------|-----|------|------|
| 單一資源 | 300s (5分鐘) | 較穩定，更新頻率低 | `cache.set(key, &user, 300)` |
| 列表查詢 | 60s (1分鐘) | 變化較頻繁 | `cache.set(key, &users, 60)` |
| 統計資料 | 30s | 需要較新資料 | `cache.set(key, &stats, 30)` |
| 熱點資料 | 600s+ | 幾乎不變 | `cache.set(key, &data, 600)` |

## 快取模式

| 模式 | 說明 | 實作方式 | 使用場景 |
|------|------|----------|----------|
| **Cache-Aside** | 應用程式管理快取 | 手動 get/set | 主要使用模式 |
| **Write-Through** | 同時寫入快取和資料庫 | 寫入時更新快取 | 需要強一致性 |
| **Write-Back** | 先寫快取，延遲寫資料庫 | 異步寫入 | 高寫入效能 |
| **Refresh-Ahead** | 預先刷新快取 | 背景任務 | 預測性載入 |

## 快取失效策略

| 策略 | 方法 | 範例 |
|------|------|------|
| TTL 過期 | 自動過期 | `cache.set(key, value, 300)` |
| 手動刪除 | 寫入時刪除 | `cache.delete("user:123").await?` |
| 模式刪除 | 批量刪除 | `redis-cli KEYS "user:*" \| xargs redis-cli DEL` |
| 版本號 | 版本控制 | `cache.set("user:123:v2", value, ttl)` |

## 快取檢查流程

| 步驟 | 操作 | 程式碼範例 |
|------|------|------------|
| 1 | 檢查快取 | `if let Some(user) = cache.get::<User>(key).await?` |
| 2 | 快取命中 | `return Ok(Json(user));` |
| 3 | 快取未命中 | 查詢資料庫 |
| 4 | 寫入快取 | `cache.set(key, &user, 300).await?` |
| 5 | 返回結果 | `Ok(Json(user))` |

## Redis 連線管理

| 配置 | 說明 | 範例 |
|------|------|------|
| 連線池 | 重用連線 | `Arc<Mutex<redis::Client>>` |
| 異步連線 | 非阻塞 I/O | `client.get_async_connection().await?` |
| 連線超時 | 避免長時間等待 | `redis::Client::open(url)?` |
| 重連機制 | 自動重連 | Redis 客戶端自動處理 |

## 序列化格式

| 格式 | 庫 | 優點 | 缺點 |
|------|------|------|------|
| JSON | `serde_json` | 可讀性高、跨語言 | 體積較大 |
| MessagePack | `rmp-serde` | 體積小、速度快 | 可讀性低 |
| Bincode | `bincode` | 極快、體積小 | 不跨語言 |

## 快取指標監控

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 快取命中率 | `hits / (hits + misses)` | > 80% |
| 快取大小 | Redis 記憶體使用 | < 80% |
| 快取延遲 | GET 操作時間 | < 1ms |
| 快取錯誤率 | 失敗請求比例 | < 0.1% |

## 最佳實踐

| 實踐 | 說明 | 範例 |
|------|------|------|
| 快取熱點資料 | 只快取頻繁訪問的資料 | 用戶資料、配置 |
| 避免快取大物件 | 限制單一快取項大小 | < 1MB |
| 使用適當的 TTL | 平衡新鮮度和效能 | 根據更新頻率設定 |
| 監控快取效能 | 追蹤命中率和延遲 | Prometheus 指標 |
| 處理快取穿透 | 防止大量未命中 | 布隆過濾器 |
| 處理快取雪崩 | 分散過期時間 | 隨機 TTL |

## 常見問題處理

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| 快取穿透 | 查詢不存在的資料 | 快取空值或使用布隆過濾器 |
| 快取雪崩 | 大量快取同時過期 | 隨機化 TTL |
| 快取擊穿 | 熱點資料過期 | 使用互斥鎖或延長 TTL |
| 資料不一致 | 快取和資料庫不同步 | 寫入時更新快取 |

## 程式碼範例

### 基本快取操作
```rust
// 取得快取
if let Some(user) = state.cache.get::<User>(&cache_key).await? {
    return Ok(Json(user));
}

// 設定快取
state.cache.set(&cache_key, &user, 300).await?;

// 刪除快取
state.cache.delete(&cache_key).await?;
```

### 快取鍵生成
```rust
// 單一資源
let cache_key = format!("user:{}", user_id);

// 列表查詢
let cache_key = format!("users:limit:{}:offset:{}", limit, offset);
```

### 快取失效
```rust
// 寫入時失效相關快取
let cache_key = format!("user:{}", user.id);
state.cache.delete(&cache_key).await?;
```

