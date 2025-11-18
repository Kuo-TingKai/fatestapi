# Rust 語法 Cheat Sheet - FastestAPI 專案

## 異步程式設計 (Async/Await)

| 語法 | 說明 | 範例 |
|------|------|------|
| `async fn` | 定義異步函數 | `async fn get_user() -> Result<User, Error>` |
| `.await` | 等待異步操作完成 | `let user = db.get_user().await?;` |
| `#[tokio::main]` | Tokio 運行時入口點 | `#[tokio::main] async fn main() {}` |
| `async {}` | 異步代碼塊 | `async { some_async_op().await }` |

## 錯誤處理 (Error Handling)

| 語法 | 說明 | 範例 |
|------|------|------|
| `Result<T, E>` | 結果類型 | `Result<String, AppError>` |
| `?` 運算符 | 錯誤傳播 | `let value = may_fail()?;` |
| `Ok(value)` | 成功值 | `Ok(user)` |
| `Err(error)` | 錯誤值 | `Err(AppError::NotFound)` |
| `#[from]` | 自動轉換錯誤 | `#[error("DB error: {0}")] Database(#[from] sqlx::Error)` |
| `unwrap_or()` | 提供預設值 | `count.unwrap_or(0)` |
| `unwrap_or_else()` | 閉包提供預設值 | `.unwrap_or_else(\|_\| "default".into())` |
| `map_err()` | 轉換錯誤類型 | `.map_err(\|_\| StatusCode::INTERNAL_SERVER_ERROR)` |

## 泛型 (Generics)

| 語法 | 說明 | 範例 |
|------|------|------|
| `<T>` | 泛型參數 | `fn get<T>(key: &str) -> Option<T>` |
| `where T: Trait` | Trait 約束 | `where T: DeserializeOwned` |
| `impl Trait` | 返回實現 Trait 的類型 | `fn get() -> impl Serialize` |
| `dyn Trait` | Trait 對象 | `Box<dyn Error>` |

## Trait 約束 (Trait Bounds)

| 語法 | 說明 | 範例 |
|------|------|------|
| `T: Serialize` | 必須實現 Serialize | `fn set<T: Serialize>(value: &T)` |
| `T: DeserializeOwned` | 必須實現 DeserializeOwned | `fn get<T: DeserializeOwned>()` |
| `T: Clone` | 必須實現 Clone | `fn clone_data<T: Clone>(data: T)` |
| `T: Send + Sync` | 多個 Trait 約束 | `T: Send + Sync + 'static` |

## 所有權與借用 (Ownership & Borrowing)

| 語法 | 說明 | 範例 |
|------|------|------|
| `&T` | 不可變借用 | `fn get(&self, key: &str)` |
| `&mut T` | 可變借用 | `fn update(&mut self, value: T)` |
| `*` | 解引用 | `*value` |
| `ref` | 模式匹配中的借用 | `if let Some(ref user) = cached` |
| `move` | 移動閉包 | `move \|x\| x + 1` |
| `.clone()` | 克隆值 | `let cloned = data.clone();` |
| `.to_string()` | 轉換為 String | `"hello".to_string()` |

## 智能指針 (Smart Pointers)

| 語法 | 說明 | 範例 |
|------|------|------|
| `Arc<T>` | 原子引用計數 | `Arc<Cache>` |
| `Arc::new()` | 創建 Arc | `Arc::new(Cache::new())` |
| `.clone()` | 克隆 Arc (增加引用) | `let shared = arc.clone();` |
| `Box<T>` | 堆分配 | `Box<dyn Error>` |
| `Rc<T>` | 引用計數 (單線程) | `Rc::new(data)` |

## 並發與同步 (Concurrency)

| 語法 | 說明 | 範例 |
|------|------|------|
| `Mutex<T>` | 互斥鎖 | `Mutex<redis::Client>` |
| `tokio::sync::Mutex` | 異步互斥鎖 | `tokio::sync::Mutex::new(data)` |
| `.lock().await` | 獲取鎖 (異步) | `let guard = mutex.lock().await;` |
| `Arc<Mutex<T>>` | 共享可變狀態 | `Arc<Mutex<Cache>>` |
| `std::sync::Arc` | 線程安全的引用計數 | `std::sync::Arc` |

## 模式匹配 (Pattern Matching)

| 語法 | 說明 | 範例 |
|------|------|------|
| `match` | 模式匹配 | `match value { Some(x) => x, None => default }` |
| `if let` | 單一模式匹配 | `if let Some(user) = cache.get() { ... }` |
| `while let` | 循環模式匹配 | `while let Some(item) = iter.next() { ... }` |
| `let` 綁定 | 解構綁定 | `let (status, body) = match error { ... }` |
| `_` | 忽略值 | `match x { Some(_) => true, None => false }` |
| `..` | 忽略剩餘字段 | `Point { x, .. }` |

## Option 處理

| 語法 | 說明 | 範例 |
|------|------|------|
| `Some(value)` | 有值 | `Some(user)` |
| `None` | 無值 | `None` |
| `.unwrap()` | 解包 (panic 如果 None) | `value.unwrap()` |
| `.unwrap_or(default)` | 解包或預設值 | `value.unwrap_or(0)` |
| `.unwrap_or_else(||)` | 解包或閉包 | `value.unwrap_or_else(\|\| 0)` |
| `.map(|x|)` | 映射 Some 值 | `value.map(|x| x * 2)` |
| `.and_then(|x|)` | 鏈式 Option | `value.and_then(|s| s.parse().ok())` |
| `?` 運算符 | Option 錯誤傳播 | `let x = maybe_value?;` |

## 迭代器 (Iterators)

| 語法 | 說明 | 範例 |
|------|------|------|
| `.iter()` | 創建迭代器 | `vec.iter()` |
| `.into_iter()` | 消費迭代器 | `vec.into_iter()` |
| `.map(|x|)` | 映射每個元素 | `.map(|row| User { ... })` |
| `.filter(|x|)` | 過濾元素 | `.filter(|x| x > 0)` |
| `.collect()` | 收集為集合 | `.collect::<Vec<_>>()` |
| `.fold(init, \|acc, x\|)` | 折疊 | `.fold(0, \|acc, x\| acc + x)` |
| `.enumerate()` | 帶索引迭代 | `.enumerate().map(|(i, x)| ...)` |

## 字串處理

| 語法 | 說明 | 範例 |
|------|------|------|
| `&str` | 字串切片 | `let s: &str = "hello";` |
| `String` | 可變字串 | `let s = String::from("hello");` |
| `format!()` | 格式化字串 | `format!("user:{}", user_id)` |
| `.to_string()` | 轉換為 String | `"hello".to_string()` |
| `&str` 轉 `String` | 使用 `.into()` | `"hello".into()` |
| `String` 轉 `&str` | 使用 `&` | `&string_value` |

## 類型轉換

| 語法 | 說明 | 範例 |
|------|------|------|
| `as` | 類型轉換 | `value as u64` |
| `.parse::<T>()` | 解析字串 | `"123".parse::<i64>()` |
| `.parse().ok()` | 解析返回 Option | `s.parse::<i64>().ok()` |
| `From` / `Into` | 類型轉換 Trait | `impl From<Error> for AppError` |
| `TryFrom` / `TryInto` | 可能失敗的轉換 | `i32::try_from(large_value)` |

## 模組系統 (Modules)

| 語法 | 說明 | 範例 |
|------|------|------|
| `mod name` | 定義模組 | `mod cache;` |
| `pub mod` | 公開模組 | `pub mod error;` |
| `use crate::` | 使用 crate 根路徑 | `use crate::error::AppError;` |
| `use super::` | 使用父模組 | `use super::User;` |
| `pub use` | 重新導出 | `pub use error::AppError;` |
| `::` | 路徑分隔符 | `std::sync::Arc` |

## 屬性 (Attributes)

| 語法 | 說明 | 範例 |
|------|------|------|
| `#[derive(...)]` | 自動實現 Trait | `#[derive(Debug, Clone, Serialize)]` |
| `#[allow(...)]` | 允許警告 | `#[allow(dead_code)]` |
| `#[cfg(...)]` | 條件編譯 | `#[cfg(test)]` |
| `#[test]` | 測試函數 | `#[test] fn test_something() {}` |
| `#[instrument]` | 追蹤裝飾器 | `#[instrument] async fn handler() {}` |
| `#[instrument(skip(state))]` | 跳過參數追蹤 | `#[instrument(skip(state))]` |
| `#[tokio::main]` | Tokio 入口點 | `#[tokio::main] async fn main() {}` |
| `#[error("...")]` | 錯誤訊息 | `#[error("Not found")]` |
| `#[from]` | 自動 From 實現 | `Database(#[from] sqlx::Error)` |

## 宏 (Macros)

| 語法 | 說明 | 範例 |
|------|------|------|
| `println!()` | 格式化輸出 | `println!("Hello {}", name);` |
| `format!()` | 格式化字串 | `format!("user:{}", id)` |
| `vec![]` | 創建向量 | `vec![1, 2, 3]` |
| `panic!()` | 恐慌 | `panic!("Something went wrong");` |
| `unreachable!()` | 不可達代碼 | `unreachable!()` |
| `todo!()` | 待辦事項 | `todo!("Implement this");` |
| `dbg!()` | 調試輸出 | `dbg!(variable);` |
| `env!()` | 編譯時環境變數 | `env!("CARGO_PKG_VERSION")` |
| `sqlx::query!()` | SQLx 查詢宏 | `sqlx::query!("SELECT * FROM users")` |
| `sqlx::query_as!()` | 查詢並映射 | `sqlx::query_as!(UserRow, "SELECT ...")` |
| `register_counter!()` | Prometheus 指標 | `register_counter!("name", "help")` |

## 結構體 (Structs)

| 語法 | 說明 | 範例 |
|------|------|------|
| `struct Name { ... }` | 定義結構體 | `struct User { id: Uuid, name: String }` |
| `pub struct` | 公開結構體 | `pub struct Cache { ... }` |
| `struct Name(...)` | 元組結構體 | `struct Point(i32, i32);` |
| `struct Name;` | 單元結構體 | `struct Marker;` |
| `impl Struct` | 實現方法 | `impl Cache { pub fn new() {} }` |
| `Self` | 結構體類型別名 | `Self { ... }` |
| `self` | 獲取所有權 | `fn consume(self) {}` |
| `&self` | 不可變借用 | `fn get(&self) {}` |
| `&mut self` | 可變借用 | `fn update(&mut self) {}` |

## 枚舉 (Enums)

| 語法 | 說明 | 範例 |
|------|------|------|
| `enum Name { ... }` | 定義枚舉 | `enum AppError { NotFound, Database }` |
| `#[derive(Error)]` | 錯誤枚舉 | `#[derive(Error, Debug)] enum AppError` |
| `match` 匹配 | 匹配枚舉變體 | `match error { AppError::NotFound => ... }` |
| `if let` | 單一變體匹配 | `if let AppError::NotFound = error { ... }` |

## 閉包 (Closures)

| 語法 | 說明 | 範例 |
|------|------|------|
| `\|x\| x + 1` | 簡單閉包 | `let add_one = \|x\| x + 1;` |
| `\|x, y\| x + y` | 多參數閉包 | `let add = \|x, y\| x + y;` |
| `move \|x\|` | 移動閉包 | `move \|x\| x + captured` |
| `\|\|` | 無參數閉包 | `\|\| default_value` |
| `\|\| -> Type` | 指定返回類型 | `\|\| -> i32 { 42 }` |

## 生命週期 (Lifetimes)

| 語法 | 說明 | 範例 |
|------|------|------|
| `'a` | 生命週期參數 | `fn get<'a>(&'a self) -> &'a str` |
| `'static` | 靜態生命週期 | `&'static str` |
| `&'a T` | 帶生命週期的引用 | `&'a User` |
| 省略規則 | 編譯器推斷 | `fn get(&self) -> &str` |

## 特徵實現 (Trait Implementation)

| 語法 | 說明 | 範例 |
|------|------|------|
| `impl Trait for Type` | 實現 Trait | `impl Serialize for User` |
| `impl Type` | 實現方法 | `impl Cache { fn new() {} }` |
| `impl<T> Trait for T` | 泛型實現 | `impl<T> Clone for Arc<T>` |
| `Default::default()` | 預設值 | `let config = AppConfig::default();` |
| `IntoResponse` | Axum 響應轉換 | `impl IntoResponse for AppError` |

## 集合類型 (Collections)

| 語法 | 說明 | 範例 |
|------|------|------|
| `Vec<T>` | 動態數組 | `Vec<User>` |
| `HashMap<K, V>` | 哈希映射 | `HashMap<String, String>` |
| `HashSet<T>` | 哈希集合 | `HashSet<u32>` |
| `Vec::new()` | 創建空向量 | `let vec = Vec::new();` |
| `vec.push()` | 添加元素 | `vec.push(item);` |
| `vec.get()` | 獲取元素 | `vec.get(0)` |
| `map.get()` | 獲取值 | `map.get("key")` |
| `map.insert()` | 插入鍵值對 | `map.insert("key", "value");` |

## 類型別名 (Type Aliases)

| 語法 | 說明 | 範例 |
|------|------|------|
| `type Name = Type` | 類型別名 | `type UserId = uuid::Uuid;` |

## 條件編譯

| 語法 | 說明 | 範例 |
|------|------|------|
| `#[cfg(test)]` | 測試配置 | `#[cfg(test)] mod tests {}` |
| `#[cfg(feature = "...")]` | 功能標誌 | `#[cfg(feature = "redis")]` |

## 常用 Trait

| Trait | 說明 | 使用場景 |
|-------|------|----------|
| `Clone` | 可克隆 | `#[derive(Clone)]` |
| `Copy` | 可複製 | 小類型如 `i32`, `bool` |
| `Debug` | 調試格式化 | `#[derive(Debug)]` |
| `Display` | 顯示格式化 | `impl Display for User` |
| `Serialize` | 序列化 | `#[derive(Serialize)]` |
| `Deserialize` | 反序列化 | `#[derive(Deserialize)]` |
| `Send` | 可跨線程發送 | `Arc<T: Send>` |
| `Sync` | 可跨線程共享引用 | `Arc<T: Sync>` |
| `From<T>` | 類型轉換 | `impl From<Error> for AppError` |
| `Into<T>` | 轉換為 T | `value.into()` |
| `IntoResponse` | Axum 響應 | `impl IntoResponse for AppError` |

## 專案特定模式

| 模式 | 說明 | 範例 |
|------|------|------|
| `Arc<Mutex<T>>` | 共享可變狀態 | `Arc<Mutex<redis::Client>>` |
| `Result<T, AppError>` | 統一錯誤類型 | `Result<User, AppError>` |
| `async fn -> Result` | 異步錯誤處理 | `async fn get() -> Result<User, AppError>` |
| `State(state): State<AppState>` | Axum 狀態注入 | `State(state): State<AppState>` |
| `Path(id): Path<Uuid>` | Axum 路徑參數 | `Path(user_id): Path<uuid::Uuid>` |
| `Query(params): Query<HashMap>` | Axum 查詢參數 | `Query(params): Query<HashMap<String, String>>` |
| `Json(payload): Json<Request>` | Axum JSON 體 | `Json(payload): Json<CreateUserRequest>` |
| `#[instrument(skip(...))]` | 跳過追蹤參數 | `#[instrument(skip(state))]` |

## 錯誤處理模式

| 模式 | 說明 | 範例 |
|------|------|------|
| `?` 運算符鏈 | 錯誤傳播 | `let user = db.get().await?;` |
| `match` 錯誤處理 | 模式匹配 | `match result { Ok(v) => v, Err(e) => return Err(e) }` |
| `if let` 錯誤處理 | 單一錯誤匹配 | `if let Err(e) = result { return Err(e); }` |
| `map_err()` | 轉換錯誤 | `.map_err(\|_\| AppError::Internal)` |
| `unwrap_or_else()` | 錯誤時執行閉包 | `.unwrap_or_else(\|e\| handle_error(e))` |

## 記憶體安全模式

| 模式 | 說明 | 範例 |
|------|------|------|
| 借用檢查 | 編譯時檢查 | `&str` 不能超過原始字串生命週期 |
| 所有權轉移 | 移動語義 | `let moved = value;` |
| 引用計數 | 共享所有權 | `Arc<T>` |
| 內部可變性 | 可變借用 | `RefCell<T>`, `Mutex<T>` |

## 效能優化技巧

| 技巧 | 說明 | 範例 |
|------|------|------|
| `&str` vs `String` | 使用切片避免分配 | `fn process(&str)` 而非 `fn process(String)` |
| `Vec::with_capacity()` | 預分配容量 | `Vec::with_capacity(100)` |
| `clone()` 最小化 | 避免不必要的克隆 | 使用引用而非克隆 |
| `Arc` 共享 | 避免深拷貝 | `Arc<Cache>` 而非 `Cache` |
| 零成本抽象 | 編譯時優化 | 泛型和 Trait 在運行時無開銷 |

## 常用標準庫

| 模組 | 用途 | 範例 |
|------|------|------|
| `std::sync::Arc` | 原子引用計數 | `Arc<T>` |
| `std::sync::Mutex` | 互斥鎖 | `Mutex<T>` |
| `std::collections` | 集合類型 | `HashMap`, `Vec` |
| `std::time::Instant` | 時間測量 | `Instant::now()` |
| `std::fmt` | 格式化 | `format!()`, `println!()` |

## 專案依賴特定語法

| 庫 | 語法 | 範例 |
|----|------|------|
| `sqlx` | `query!()` 宏 | `sqlx::query!("SELECT * FROM users")` |
| `sqlx` | `query_as!()` 宏 | `sqlx::query_as!(UserRow, "SELECT ...")` |
| `serde` | `#[derive(Serialize)]` | `#[derive(Serialize, Deserialize)]` |
| `thiserror` | `#[derive(Error)]` | `#[derive(Error, Debug)]` |
| `axum` | `State` extractor | `State(state): State<AppState>` |
| `tracing` | `#[instrument]` | `#[instrument] async fn handler() {}` |
| `prometheus` | `register_counter!()` | `register_counter!("name", "help")` |

## 快速參考

### 常見錯誤處理
```rust
// 使用 ? 運算符
let value = may_fail()?;

// 提供預設值
let value = may_fail().unwrap_or(default);

// 轉換錯誤
let value = may_fail().map_err(|e| AppError::from(e))?;
```

### 常見 Option 處理
```rust
// 解包或預設值
let value = option.unwrap_or(0);

// 鏈式處理
let value = option.and_then(|s| s.parse().ok()).unwrap_or(0);

// 模式匹配
match option {
    Some(value) => value,
    None => default,
}
```

### 常見異步模式
```rust
// 異步函數
async fn get_user() -> Result<User, Error> {
    let user = db.get().await?;
    Ok(user)
}

// 並發執行
let (user, stats) = tokio::join!(
    db.get_user(),
    db.get_stats()
);
```

### 常見泛型模式
```rust
// 泛型函數
fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Error> {
    // ...
}

// Trait 約束
where
    T: Serialize + Send + Sync + 'static
```

