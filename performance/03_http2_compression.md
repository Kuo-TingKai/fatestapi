# HTTP/2 和壓縮支援 Cheat Sheet

## HTTP/2 特性

| 特性 | 說明 | 優勢 |
|------|------|------|
| **多路復用** | 單一連線處理多個請求 | 減少連線開銷 |
| **伺服器推送** | 伺服器主動推送資源 | 減少往返次數 |
| **標頭壓縮** | HPACK 壓縮標頭 | 減少傳輸大小 |
| **二進位協議** | 二進位格式 | 解析更快 |

## HTTP/2 配置

| 配置項 | 位置 | 說明 | 範例 |
|--------|------|------|------|
| HTTP/2 啟用 | NGINX | 需要 SSL 憑證 | `listen 443 ssl http2;` |
| 多路復用 | 自動 | HTTP/2 預設啟用 | - |
| 伺服器推送 | NGINX | 預先推送資源 | `http2_push /style.css;` |

## 壓縮演算法比較

| 演算法 | 壓縮比 | 速度 | CPU 使用 | 支援 |
|--------|--------|------|---------|------|
| **Gzip** | 中等 | 快 | 低 | 廣泛支援 |
| **Brotli** | 高 | 中等 | 中等 | 現代瀏覽器 |
| **Zstandard** | 高 | 快 | 低 | 較少支援 |
| **Deflate** | 低 | 快 | 低 | 廣泛支援 |

## NGINX Gzip 壓縮配置

| 指令 | 說明 | 範例 |
|------|------|------|
| `gzip on;` | 啟用 Gzip | `gzip on;` |
| `gzip_vary on;` | 添加 Vary 標頭 | `gzip_vary on;` |
| `gzip_comp_level` | 壓縮級別 (1-9) | `gzip_comp_level 6;` |
| `gzip_types` | 壓縮的 MIME 類型 | `gzip_types text/plain application/json;` |
| `gzip_min_length` | 最小壓縮長度 | `gzip_min_length 1000;` |
| `gzip_proxied` | 代理請求壓縮 | `gzip_proxied any;` |

## Axum 壓縮中間件

| 中間件 | 功能 | 配置 |
|--------|------|------|
| `CompressionLayer` | Gzip 壓縮 | `.layer(CompressionLayer::new())` |
| `CompressionLayer::brotli()` | Brotli 壓縮 | `.layer(CompressionLayer::brotli())` |
| `CompressionLayer::gzip()` | Gzip 壓縮 | `.layer(CompressionLayer::gzip())` |

## 壓縮配置最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **壓縮級別** | 平衡壓縮比和 CPU | 6 (Gzip) |
| **壓縮類型** | 只壓縮文字類型 | JSON, HTML, CSS, JS |
| **最小長度** | 避免壓縮小檔案 | 1000 bytes |
| **快取壓縮結果** | 避免重複壓縮 | NGINX 自動處理 |

## MIME 類型壓縮建議

| MIME 類型 | 是否壓縮 | 理由 |
|-----------|----------|------|
| `text/plain` | ✅ | 文字檔案壓縮效果好 |
| `text/html` | ✅ | HTML 壓縮效果好 |
| `text/css` | ✅ | CSS 壓縮效果好 |
| `text/javascript` | ✅ | JS 壓縮效果好 |
| `application/json` | ✅ | JSON 壓縮效果好 |
| `application/xml` | ✅ | XML 壓縮效果好 |
| `image/*` | ❌ | 圖片已壓縮 |
| `video/*` | ❌ | 影片已壓縮 |
| `application/octet-stream` | ❌ | 二進位檔案 |

## HTTP/2 伺服器推送

| 場景 | 說明 | 範例 |
|------|------|------|
| CSS 推送 | 推送樣式表 | `http2_push /style.css;` |
| JS 推送 | 推送腳本 | `http2_push /app.js;` |
| 圖片推送 | 推送關鍵圖片 | `http2_push /logo.png;` |

## 壓縮效能指標

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 壓縮比 | 原始大小 / 壓縮後大小 | > 70% |
| 壓縮時間 | 壓縮處理時間 | < 10ms |
| CPU 使用率 | 壓縮 CPU 開銷 | < 5% |
| 傳輸大小減少 | 節省的頻寬 | > 50% |

## NGINX 壓縮配置範例

```nginx
# Gzip 壓縮配置
gzip on;
gzip_vary on;
gzip_proxied any;
gzip_comp_level 6;
gzip_types 
    text/plain 
    text/css 
    text/xml 
    text/javascript 
    application/json 
    application/javascript 
    application/xml+rss 
    application/rss+xml 
    font/truetype 
    font/opentype 
    application/vnd.ms-fontobject 
    image/svg+xml;
gzip_min_length 1000;
```

## Axum 壓縮實作

| 實作方式 | 程式碼 | 說明 |
|----------|--------|------|
| 基本壓縮 | `.layer(CompressionLayer::new())` | 自動選擇最佳壓縮 |
| Gzip 壓縮 | `.layer(CompressionLayer::gzip())` | 僅使用 Gzip |
| Brotli 壓縮 | `.layer(CompressionLayer::brotli())` | 僅使用 Brotli |
| 自訂壓縮 | `.layer(CompressionLayer::new().gzip(BrotliLevel::default()))` | 自訂配置 |

## 壓縮層級選擇

| 級別 | 壓縮比 | 速度 | CPU | 使用場景 |
|------|--------|------|-----|----------|
| 1-3 | 低 | 最快 | 最低 | 即時壓縮 |
| 4-6 | 中等 | 快 | 低 | **推薦** (平衡) |
| 7-9 | 高 | 慢 | 高 | 離線壓縮 |

## HTTP/2 vs HTTP/1.1

| 特性 | HTTP/1.1 | HTTP/2 |
|------|----------|--------|
| 連線數 | 多個連線 | 單一連線 |
| 多路復用 | ❌ | ✅ |
| 標頭壓縮 | ❌ | ✅ (HPACK) |
| 伺服器推送 | ❌ | ✅ |
| 二進位協議 | ❌ | ✅ |

## 壓縮策略

| 策略 | 說明 | 適用場景 |
|------|------|----------|
| **即時壓縮** | 請求時壓縮 | 動態內容 |
| **預壓縮** | 預先壓縮靜態檔案 | 靜態內容 |
| **條件壓縮** | 根據條件決定 | 混合內容 |
| **分級壓縮** | 不同級別壓縮 | 根據內容類型 |

## 最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **啟用 HTTP/2** | 需要 SSL 憑證 | 生產環境必備 |
| **使用 Gzip** | 廣泛支援 | 預設選擇 |
| **壓縮級別 6** | 平衡效能和壓縮比 | 推薦設定 |
| **只壓縮文字** | 避免壓縮已壓縮內容 | 圖片、影片不壓縮 |
| **設定最小長度** | 避免壓縮小檔案 | 1000 bytes |
| **監控壓縮效果** | 追蹤壓縮比和 CPU | Prometheus 指標 |

## 常見問題

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| 壓縮未生效 | 內容類型未包含 | 檢查 gzip_types |
| CPU 使用率高 | 壓縮級別太高 | 降低 gzip_comp_level |
| 壓縮比低 | 內容已壓縮 | 跳過已壓縮類型 |
| HTTP/2 未啟用 | 缺少 SSL | 配置 SSL 憑證 |

## 效能測試

| 測試項目 | 方法 | 目標 |
|----------|------|------|
| 壓縮比 | 比較原始和壓縮大小 | > 70% |
| 傳輸時間 | 比較壓縮前後傳輸時間 | 減少 > 50% |
| CPU 使用 | 監控壓縮 CPU 開銷 | < 5% |
| 記憶體使用 | 監控壓縮記憶體 | 穩定 |

