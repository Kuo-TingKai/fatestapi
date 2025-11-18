# 非同步 I/O (Tokio) Cheat Sheet

## 非同步 I/O 概念

| 概念 | 說明 | 優勢 |
|------|------|------|
| **非阻塞 I/O** | 不等待 I/O 完成 | 高並發處理 |
| **事件循環** | 單線程處理多任務 | 減少線程開銷 |
| **Future** | 異步計算的抽象 | 組合異步操作 |
| **Async/Await** | 異步程式設計語法 | 簡化異步程式碼 |

## Tokio 核心組件

| 組件 | 說明 | 用途 |
|------|------|------|
| **Runtime** | 異步運行時 | 執行異步任務 |
| **Task** | 異步任務 | 輕量級執行單元 |
| **Spawner** | 任務生成器 | 創建新任務 |
| **Reactor** | 事件反應器 | 處理 I/O 事件 |

## Tokio Runtime 配置

| 配置項 | 說明 | 預設值 | 建議值 |
|--------|------|--------|--------|
| `worker_threads` | Worker 線程數 | CPU 核心數 | 4-8 |
| `max_blocking_threads` | 阻塞線程數 | 512 | 100-200 |
| `thread_stack_size` | 線程堆疊大小 | 2MB | 2MB |
| `thread_keep_alive` | 線程保持時間 | 10s | 10s |

## Tokio 啟動方式

| 方式 | 說明 | 範例 |
|------|------|------|
| `#[tokio::main]` | 屬性宏啟動 | `#[tokio::main] async fn main() {}` |
| `tokio::runtime::Runtime::new()` | 手動創建 | `Runtime::new().unwrap().block_on(async {})` |
| `tokio::spawn()` | 生成任務 | `tokio::spawn(async { ... })` |

## 異步函數定義

| 語法 | 說明 | 範例 |
|------|------|------|
| `async fn` | 異步函數 | `async fn get_user() -> Result<User>` |
| `.await` | 等待異步操作 | `let user = get_user().await?;` |
| `async {}` | 異步代碼塊 | `async { some_op().await }` |
| `async move {}` | 移動異步塊 | `async move { ... }` |

## 並發執行

| 方法 | 說明 | 範例 |
|------|------|------|
| `tokio::join!` | 並發執行多個 Future | `tokio::join!(f1, f2, f3)` |
| `tokio::try_join!` | 並發執行，錯誤時停止 | `tokio::try_join!(f1, f2)` |
| `futures::join!` | 通用並發執行 | `futures::join!(f1, f2)` |
| `futures::try_join!` | 通用並發，錯誤處理 | `futures::try_join!(f1, f2)` |

## 任務管理

| 操作 | 方法 | 說明 |
|------|------|------|
| 生成任務 | `tokio::spawn()` | 在背景執行 |
| 等待任務 | `task.await` | 等待任務完成 |
| 取消任務 | `task.abort()` | 取消任務 |
| 任務句柄 | `JoinHandle` | 管理任務 |

## 異步 I/O 操作

| 操作 | 方法 | 範例 |
|------|------|------|
| TCP 連線 | `TcpStream::connect()` | `TcpStream::connect("127.0.0.1:8080").await?` |
| TCP 監聽 | `TcpListener::bind()` | `TcpListener::bind("0.0.0.0:8080").await?` |
| 檔案讀取 | `tokio::fs::read()` | `tokio::fs::read("file.txt").await?` |
| 檔案寫入 | `tokio::fs::write()` | `tokio::fs::write("file.txt", data).await?` |
| 定時器 | `tokio::time::sleep()` | `tokio::time::sleep(Duration::from_secs(1)).await` |

## 同步原語

| 類型 | 說明 | 範例 |
|------|------|------|
| `tokio::sync::Mutex` | 異步互斥鎖 | `Mutex::new(data)` |
| `tokio::sync::RwLock` | 讀寫鎖 | `RwLock::new(data)` |
| `tokio::sync::Semaphore` | 信號量 | `Semaphore::new(10)` |
| `tokio::sync::mpsc` | 通道 | `mpsc::channel(100)` |
| `tokio::sync::oneshot` | 單次通道 | `oneshot::channel()` |

## 異步資料庫操作

| 操作 | 方法 | 範例 |
|------|------|------|
| 連線池 | `PgPool::connect()` | `PgPool::connect(url).await?` |
| 執行查詢 | `.execute()` | `query.execute(pool).await?` |
| 查詢單筆 | `.fetch_one()` | `query.fetch_one(pool).await?` |
| 查詢多筆 | `.fetch_all()` | `query.fetch_all(pool).await?` |
| 可選查詢 | `.fetch_optional()` | `query.fetch_optional(pool).await?` |

## 異步 Redis 操作

| 操作 | 方法 | 範例 |
|------|------|------|
| 建立連線 | `Client::open()` | `redis::Client::open(url)?` |
| 異步連線 | `.get_async_connection()` | `client.get_async_connection().await?` |
| GET | `.get()` | `conn.get("key").await?` |
| SET | `.set()` | `conn.set("key", "value").await?` |
| SETEX | `.set_ex()` | `conn.set_ex("key", "value", 60).await?` |

## 效能優化技巧

| 技巧 | 說明 | 效果 |
|------|------|------|
| **批量操作** | 合併多個操作 | 減少往返次數 |
| **連線池** | 重用連線 | 減少連線開銷 |
| **並發執行** | 同時執行多個操作 | 提高吞吐量 |
| **避免阻塞** | 使用異步版本 | 不阻塞事件循環 |
| **任務優先級** | 設定任務優先級 | 重要任務優先 |

## 常見模式

| 模式 | 說明 | 範例 |
|------|------|------|
| **並發查詢** | 同時查詢多個資料 | `tokio::join!(db1, db2)` |
| **超時處理** | 設定操作超時 | `tokio::time::timeout()` |
| **重試機制** | 失敗時重試 | `tokio::time::sleep().await` |
| **背景任務** | 背景執行任務 | `tokio::spawn()` |

## 錯誤處理

| 模式 | 說明 | 範例 |
|------|------|------|
| `?` 運算符 | 錯誤傳播 | `let result = op().await?;` |
| `map_err()` | 轉換錯誤 | `.await.map_err(|e| ...)` |
| `unwrap_or()` | 提供預設值 | `.await.unwrap_or(default)` |

## 程式碼範例

### 基本異步函數
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let user = get_user().await?;
    Ok(())
}

async fn get_user() -> Result<User> {
    let user = db.get_user().await?;
    Ok(user)
}
```

### 並發執行
```rust
// 並發執行多個操作
let (user, stats) = tokio::join!(
    db.get_user(id),
    db.get_stats()
);
```

### 異步互斥鎖
```rust
let mutex = Arc::new(tokio::sync::Mutex::new(data));
let guard = mutex.lock().await;
// 使用 guard
```

### 超時處理
```rust
match tokio::time::timeout(
    Duration::from_secs(5),
    slow_operation()
).await {
    Ok(result) => result,
    Err(_) => Err("Timeout"),
}
```

## 效能指標

| 指標 | 說明 | 目標值 |
|------|------|--------|
| 並發連線數 | 同時處理的連線 | > 10,000 |
| 請求延遲 | P95 延遲 | < 10ms |
| CPU 使用率 | CPU 使用情況 | < 80% |
| 記憶體使用 | 記憶體使用情況 | 穩定 |

## 最佳實踐

| 實踐 | 說明 | 建議 |
|------|------|------|
| **避免阻塞** | 不使用阻塞操作 | 使用異步版本 |
| **合理並發** | 控制並發數量 | 使用 Semaphore |
| **錯誤處理** | 適當處理錯誤 | 使用 `?` 或 `map_err` |
| **資源管理** | 正確釋放資源 | 使用 RAII |
| **監控任務** | 追蹤任務狀態 | Prometheus 指標 |

## 常見問題

| 問題 | 原因 | 解決方案 |
|------|------|----------|
| **任務洩漏** | 任務未正確完成 | 檢查任務句柄 |
| **死鎖** | 鎖順序問題 | 統一鎖順序 |
| **記憶體洩漏** | 循環引用 | 使用 `Arc` 和 `Weak` |
| **阻塞事件循環** | 使用阻塞操作 | 使用異步版本 |

## Tokio 特性

| 特性 | 說明 | 啟用方式 |
|------|------|----------|
| `full` | 所有特性 | `features = ["full"]` |
| `rt` | 運行時 | `features = ["rt"]` |
| `net` | 網路功能 | `features = ["net"]` |
| `fs` | 檔案系統 | `features = ["fs"]` |
| `time` | 時間功能 | `features = ["time"]` |
| `sync` | 同步原語 | `features = ["sync"]` |

