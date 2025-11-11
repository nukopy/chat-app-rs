# Flaky Integration Tests の修正

## 概要

### 目的

- HTTP API 統合テスト（`tests/http_api.rs`）の Flaky Test を修正する
- テストの安定性を向上させ、CI/CD で信頼できるテスト結果を得る

### 背景

現在、`cargo test --test http_api` が 2 回に 1 回程度失敗する（Flaky Test）。
エラーメッセージは `Connection refused` で、サーバーが起動する前にテストがリクエストを送信してしまう。

```sh
error: reqwest::Error { kind: Request, url: "http://127.0.0.1:19080/api/health",
source: ConnectError("tcp connect error", Os { code: 61, kind: ConnectionRefused }) }
```

### スコープ

- `tests/fixtures/mod.rs` の `TestServer::start()` メソッドを修正
- サーバーが完全に起動するまで待機する仕組みを実装
- 既存のテストコード（`tests/http_api.rs`）は変更不要

## 方針

### 根本原因の整理

#### タイムライン図

```sh
ケース 1（成功）:
0ms    ─ spawn server process
500ms  ─ server が起動完了、ポート 19080 をリッスン開始
1000ms ─ sleep終了、テストが HTTP リクエスト送信 ✅ 成功

ケース 2（失敗）:
0ms    ─ spawn server process
...    ─ (CPU が遅い、コンパイルに時間がかかる等)
1000ms ─ sleep終了、テストが HTTP リクエスト送信
1200ms ─ server がようやく起動完了 ❌ Connection refused
```

#### 根本原因

1. **固定待機時間**：`thread::sleep(1000ms)` は環境に依存
   - 速いマシン: 500ms で起動 → 500ms 無駄に待つ
   - 遅いマシン: 1500ms かかる → テスト失敗

2. **起動確認なし**：サーバーが「実際に起動したか」を確認していない
   - プロセスが spawn されただけでは不十分
   - ポートがリッスン状態になるまで時間がかかる

3. **Cargo のコンパイル時間**：`cargo run` はビルドも含む
   - 初回実行: コンパイル必要 → 数秒
   - 2回目以降: キャッシュあり → 速い
   - **これが「2回に1回」失敗する理由**

### アプローチ

**選択肢 2: ヘルスチェックポーリング（推奨）**:

サーバーが実際に `/api/health` エンドポイントに応答するまでリトライする。

```rust
// サーバーが実際に応答するまでリトライ
let max_retries = 30;  // 最大 3 秒
for _ in 0..max_retries {
    thread::sleep(Duration::from_millis(100));
    if let Ok(response) = reqwest::blocking::get(format!("{}/api/health", base_url)) {
        if response.status().is_success() {
            return;  // 準備完了
        }
    }
}
panic!("Server failed to start within timeout");
```

**メリット**:

- ✅ 確実に起動を待つ
- ✅ 最小限の待機時間（起動したらすぐ進む）
- ✅ サーバーが完全に準備完了したことを保証

**必要な依存追加**:

- `reqwest` の `blocking` feature を有効化

### 品質基準

- `cargo test --test http_api` を 10 回連続実行してすべて成功
- テスト実行時間が大幅に増えない（最大 3 秒以内）
- 既存のテストコードに変更不要

## タスク

### Phase 1: 依存関係の追加

- [x] ~~`Cargo.toml` に `reqwest` の `blocking` feature を追加~~
  - 注: 最終的には `blocking` feature は不要だった（async/await を使用）

### Phase 2: TestServer::start() の修正

- [x] `tests/fixtures/mod.rs` の `TestServer::start()` を修正
  - [x] 固定待機 `thread::sleep(1000ms)` を削除
  - [x] ヘルスチェックポーリングロジックを追加（async/await 使用）
  - [x] 最大リトライ回数を 30 回（約 3 秒）に設定
  - [x] タイムアウト時は panic してエラーメッセージを表示
  - [x] Clippy の zombie_processes 警告に対応

### Phase 3: すべての統合テストファイルを async/await に変換

- [x] `tests/http_api.rs` を修正（4箇所に `.await` 追加）
- [x] `tests/websocket_connection.rs` を修正（4テスト関数を async 化）
- [x] `tests/websocket_messaging.rs` を修正（2テスト関数を async 化）

### Phase 4: 検証と品質確認

- [x] `cargo test --test http_api` を 10 回連続実行
  - [x] すべて成功（実行時間: 0.49〜0.59秒）
- [x] 他の統合テストも動作確認
  - [x] `cargo test --test websocket_connection` - 成功
  - [x] `cargo test --test websocket_messaging` - 成功
- [x] `cargo fmt` - 成功
- [x] `cargo clippy --all-targets --all-features` - 成功
- [x] `cargo test` (全テスト) - 80件すべて成功

## 進捗状況

- **開始日**: 2025-11-12 00:00:00 JST
- **完了日**: 2025-11-12 01:30:00 JST（推定）
- **ステータス**: ✅ **完了**
- **現在のフェーズ**: すべてのフェーズ完了
- **完了タスク数**: 16/16
- **実装時間**: 約 1.5時間
- **最終結果**:
  - HTTP API テスト 10回連続実行: すべて成功（0.49〜0.59秒）
  - 全統合テスト（11件）: すべて成功
  - 全ユニットテスト（69件）: すべて成功
  - Clippy: 警告なし
  - テスト実行時間: 最大 2.5秒（目標 3秒以内を達成）

## 備考

### Flaky Test とは

**Flaky Test**（フレークテスト）は、実行するたびに成功/失敗が変わる不安定なテストのこと。
原因は通常、以下のいずれか：

- タイミング依存（今回のケース）
- 並列実行時の競合状態
- 外部サービスへの依存
- テストの順序依存

### 用語整理

- **統合テスト（Integration Test）**: 複数のコンポーネントを組み合わせてテスト
- **E2E テスト（End-to-End Test）**: システム全体を外部から動作確認
- **Flaky Test**: 実行するたびに成功/失敗が変わる不安定なテスト

今回は **Integration Test** の **Flaky Test** 問題を解決します。

### 参考資料

- [Testing - The Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Integration testing - Rust by Example](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)

## 実装の詳細

### 実装したヘルスチェックポーリングロジック

```rust
/// Start a test server on the specified port
#[allow(clippy::zombie_processes)] // Process is properly handled in Drop and panic paths
pub async fn start(port: u16) -> Self {
    let process = Command::new("cargo")
        .args(["run", "--bin", "server", "--", "--port", &port.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server");

    // Wait for server to be ready by polling health endpoint
    let health_url = format!("http://127.0.0.1:{}/api/health", port);
    let max_retries = 30; // 30 * 100ms = 3 seconds max

    for attempt in 1..=max_retries {
        tokio::time::sleep(Duration::from_millis(100)).await;

        if let Ok(response) = reqwest::get(&health_url).await
            && response.status().is_success()
        {
            // Server is ready
            return TestServer { process, port };
        }

        // On last attempt, provide helpful error message
        if attempt == max_retries {
            // Kill the process before panicking to avoid zombie process
            let mut process = process;
            let _ = process.kill();
            let _ = process.wait();
            panic!(
                "Server failed to start within {} seconds. \
                 Health endpoint {} did not respond successfully.",
                max_retries as f64 / 10.0,
                health_url
            );
        }
    }

    unreachable!()
}
```

### ハマりポイントと解決策

#### 1. `reqwest::blocking` を使うとランタイムエラー

**問題**:
最初は `reqwest::blocking::get()` を使って実装したが、`#[tokio::test]` の async コンテキスト内で実行すると以下のエラーが発生：

```
Cannot drop a runtime in a context where blocking is not allowed.
```

**原因**:
Tokio の非同期ランタイム内で blocking 操作を実行すると、ランタイムがデッドロックする可能性があるため禁止されている。

**解決策**:

- `TestServer::start()` を `async fn` に変更
- `reqwest::blocking` ではなく `reqwest` (async 版) を使用
- `thread::sleep` を `tokio::time::sleep` に変更
- すべてのテストファイルで `TestServer::start()` に `.await` を追加

#### 2. Clippy の `zombie_processes` 警告

**問題**:
Clippy が「プロセスがすべてのコードパスで `wait()` されていない」という警告を出す。

**原因**:
Clippy は静的解析でコードを確認するため、以下のような複雑なフローを理解できない：

- 成功時: プロセスは `TestServer` に移動し、`Drop` で自動的にクリーンアップ
- タイムアウト時: 明示的に `kill()` と `wait()` を実行してから `panic!`
- リトライ時: プロセスは生きたまま（意図的）

**解決策**:
実装が正しいことを確認した上で、`#[allow(clippy::zombie_processes)]` 属性を追加してこの警告を抑制。

#### 3. すべての統合テストファイルで変更が必要

**問題**:
当初は `tests/http_api.rs` のみの修正を想定していたが、`TestServer::start()` を async にしたため、この関数を使っているすべてのテストファイルで修正が必要になった。

**影響を受けたファイル**:

- `tests/http_api.rs` (4箇所)
- `tests/websocket_connection.rs` (4テスト関数)
- `tests/websocket_messaging.rs` (2テスト関数)

**対応**:
すべてのテスト関数を `#[test]` → `#[tokio::test]` に変更し、`async fn` として `.await` を追加。

### 成果

**Before (固定待機)**:

```rust
thread::sleep(Duration::from_millis(1000));  // 常に 1秒待つ
```

- 実行時間: 常に 1秒以上
- 成功率: 約 50%（環境依存）

**After (ヘルスチェックポーリング)**:

```rust
// サーバーが起動したらすぐ進む（最大 3秒）
for attempt in 1..=30 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    if server_is_ready { return; }
}
```

- 実行時間: 0.49〜0.59秒（約 50% 高速化）
- 成功率: 100%（10回連続成功）
- 環境に依存しない安定性

### 学んだこと

1. **Tokio の非同期ランタイムで blocking 操作は禁止**
   - `reqwest::blocking` は同期コードでのみ使用可能
   - async コンテキストでは async 版を使う

2. **Clippy の警告は無視せず理解する**
   - `zombie_processes` 警告は実際の問題を指摘している
   - 抑制する場合は、正しさを確認してコメントを残す

3. **テストの待機は「固定時間」ではなく「状態確認」**
   - `sleep()` は最後の手段
   - 可能な限りポーリングや通知を使う
   - 最大待機時間（タイムアウト）は必ず設定する

4. **統合テストは相互依存に注意**
   - 共有フィクスチャ（`TestServer`, `TestClient`）の変更は影響範囲が広い
   - 修正前にすべての使用箇所を確認する
