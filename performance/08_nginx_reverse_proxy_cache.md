# NGINX 反向代理快取 Cheat Sheet

## 反向代理概念

| 概念 | 說明 | 優勢 |
|------|------|------|
| **反向代理** | 代理伺服器轉發請求 | 負載平衡、快取 |
| **上游伺服器** | 後端應用伺服器 | 多個實例分散負載 |
| **快取層** | 快取響應內容 | 減少後端壓力 |
| **負載平衡** | 分散請求到多個伺服器 | 提高可用性 |

## NGINX 反向代理配置

| 指令 | 說明 | 範例 |
|------|------|------|
| `proxy_pass` | 轉發請求 | `proxy_pass http://api_backend;` |
| `proxy_set_header` | 設定代理標頭 | `proxy_set_header Host $host;` |
| `proxy_http_version` | HTTP 版本 | `proxy_http_version 1.1;` |
| `proxy_connect_timeout` | 連線超時 | `proxy_connect_timeout 5s;` |
| `proxy_send_timeout` | 發送超時 | `proxy_send_timeout 10s;` |
| `proxy_read_timeout` | 讀取超時 | `proxy_read_timeout 10s;` |

## 上游伺服器配置

| 指令 | 說明 | 範例 |
|------|------|------|
| `upstream` | 定義上游伺服器組 | `upstream api_backend { ... }` |
| `server` | 上游伺服器 | `server host.docker.internal:3000;` |
| `least_conn` | 最少連線演算法 | `least_conn;` |
| `keepalive` | 保持連線 | `keepalive 32;` |
| `max_fails` | 最大失敗次數 | `max_fails=3` |
| `fail_timeout` | 失敗超時 | `fail_timeout=30s` |

## 快取配置

| 指令 | 說明 | 範例 |
|------|------|------|
| `proxy_cache_path` | 快取路徑配置 | `proxy_cache_path /var/cache/nginx ...` |
| `proxy_cache` | 啟用快取 | `proxy_cache api_cache;` |
| `proxy_cache_valid` | 快取有效期 | `proxy_cache_valid 200 60s;` |
| `proxy_cache_key` | 快取鍵 | `proxy_cache_key "$scheme$request_method$host$request_uri";` |
| `proxy_cache_use_stale` | 使用過期快取 | `proxy_cache_use_stale error timeout;` |
| `proxy_cache_background_update` | 背景更新 | `proxy_cache_background_update on;` |

## 快取路徑配置

| 參數 | 說明 | 範例 |
|------|------|------|
| `levels` | 目錄層級 | `levels=1:2` |
| `keys_zone` | 鍵區域 | `keys_zone=api_cache:10m` |
| `max_size` | 最大大小 | `max_size=1g` |
| `inactive` | 非活動時間 | `inactive=60m` |
| `use_temp_path` | 使用臨時路徑 | `use_temp_path=off` |

## 快取狀態

| 狀態 | 說明 | 標頭值 |
|------|------|--------|
| **MISS** | 快取未命中 | `X-Cache-Status: MISS` |
| **HIT** | 快取命中 | `X-Cache-Status: HIT` |
| **BYPASS** | 跳過快取 | `X-Cache-Status: BYPASS` |
| **EXPIRED** | 快取過期 | `X-Cache-Status: EXPIRED` |
| **STALE** | 使用過期快取 | `X-Cache-Status: STALE` |
| **UPDATING** | 更新中 | `X-Cache-Status: UPDATING` |

## 快取控制

| 指令 | 說明 | 範例 |
|------|------|------|
| `proxy_cache_bypass` | 跳過快取條件 | `proxy_cache_bypass $http_pragma;` |
| `proxy_no_cache` | 不緩存條件 | `proxy_no_cache $http_pragma;` |
| `add_header` | 添加標頭 | `add_header X-Cache-Status $upstream_cache_status;` |

## 負載平衡演算法

| 演算法 | 指令 | 說明 |
|--------|------|------|
| **輪詢** | `round-robin` (預設) | 輪流分配 |
| **最少連線** | `least_conn` | 分配給連線數最少的伺服器 |
| **IP 雜湊** | `ip_hash` | 根據 IP 分配 |
| **權重** | `weight=n` | 根據權重分配 |

## 程式碼範例

### 基本反向代理
```nginx
upstream api_backend {
    least_conn;
    server host.docker.internal:3000 max_fails=3 fail_timeout=30s;
    keepalive 32;
}

server {
    location / {
        proxy_pass http://api_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### 快取配置
```nginx
# 快取路徑配置 (必須在 http 區塊)
proxy_cache_path /var/cache/nginx 
    levels=1:2 
    keys_zone=api_cache:10m 
    max_size=1g 
    inactive=60m 
    use_temp_path=off;

server {
    location / {
        proxy_pass http://api_backend;
        proxy_cache api_cache;
        proxy_cache_valid 200 60s;
        proxy_cache_use_stale error timeout updating http_500 http_502 http_503 http_504;
        proxy_cache_background_update on;
        add_header X-Cache-Status $upstream_cache_status;
    }
}
```

### 條件快取
```nginx
location / {
    proxy_pass http://api_backend;
    
    # 只快取 GET 請求
    proxy_cache api_cache;
    proxy_cache_methods GET HEAD;
    
    # 根據狀態碼設定快取時間
    proxy_cache_valid 200 302 60s;
    proxy_cache_valid 404 1m;
    
    # 跳過特定條件
    proxy_cache_bypass $http_pragma $http_authorization;
}
```

## 快取最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **快取靜態內容** | 快取不常變化的內容 | GET 請求、200 狀態碼 |
| **適當的 TTL** | 根據內容更新頻率設定 | 60s-300s |
| **快取鍵設計** | 包含必要的識別資訊 | URL、方法、標頭 |
| **監控快取命中率** | 追蹤快取效果 | 目標 > 80% |
| **處理快取失效** | 適時清除快取 | 手動或自動清除 |

## 快取策略

| 策略 | 說明 | 適用場景 |
|------|------|----------|
| **全部快取** | 快取所有響應 | 靜態內容 |
| **選擇性快取** | 只快取特定內容 | 動態內容 |
| **條件快取** | 根據條件決定 | 混合內容 |
| **不快取** | 跳過快取 | 敏感資料、即時資料 |

## 快取失效

| 方法 | 說明 | 實作 |
|------|------|------|
| **TTL 過期** | 時間到期自動失效 | `proxy_cache_valid` |
| **手動清除** | 手動刪除快取 | `rm -rf /var/cache/nginx/*` |
| **版本控制** | 使用版本號 | URL 包含版本 |
| **標頭控制** | 使用 Cache-Control | `Cache-Control: no-cache` |

## 效能優化

| 優化項 | 說明 | 效果 |
|--------|------|------|
| **keepalive** | 保持連線 | 減少連線開銷 |
| **快取命中** | 快取響應 | 減少後端負載 |
| **背景更新** | 背景更新快取 | 不阻塞請求 |
| **使用過期快取** | 允許使用過期快取 | 提高可用性 |

## 監控指標

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 快取命中率 | HIT / (HIT + MISS) | > 80% |
| 快取大小 | 快取使用空間 | < 80% max_size |
| 快取效率 | 節省的後端請求 | > 70% |
| 快取延遲 | 快取響應時間 | < 1ms |

## 常見問題

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| **快取未生效** | 配置錯誤 | 檢查 `proxy_cache` 指令 |
| **快取命中率低** | TTL 太短或內容變化快 | 調整 TTL 或快取策略 |
| **快取過大** | max_size 設定過大 | 調整 max_size |
| **快取不一致** | 快取未及時更新 | 使用版本控制或手動清除 |

## 安全考量

| 考量 | 說明 | 建議 |
|------|------|------|
| **敏感資料** | 不快取敏感資訊 | 使用 `proxy_no_cache` |
| **認證資訊** | 不快取需要認證的內容 | 檢查 `Authorization` 標頭 |
| **個人資料** | 不快取個人資料 | 根據內容類型決定 |

## 進階配置

| 配置 | 說明 | 範例 |
|------|------|------|
| **多層快取** | 多個快取區域 | 不同路徑使用不同快取 |
| **快取分片** | 分散快取 | 使用多個 keys_zone |
| **快取預熱** | 預先載入快取 | 背景任務載入 |
| **智慧快取** | 根據內容決定 | 使用 Lua 腳本 |

