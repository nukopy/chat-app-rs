# Cargo Workspace への移行

## 概要

### 目的

単一クレート構成から Cargo Workspace 構成へ移行し、server と client を明確に分離する。

### 背景

- プロジェクトが成長し、単一クレート構成では管理が困難になってきた
- server の Layered Architecture と client のシンプルな構造が混在
- Phase 2 のドメインモデル拡張、Phase 3 の TUI クライアント追加を控えている
- ビルドとテストの効率化が必要

### スコープ

- workspace 構造の作成（packages/shared, packages/server, packages/client）
- 既存コードの各パッケージへの移動
- import パスの修正
- 統合テストの修正
- テストと動作確認

**スコープ外**:

- 新機能の追加
- ドメインモデルの拡張（Phase 2 で実施）
- TUI の実装（Phase 3 で実施）

### 参照

- [ADR 0002: Cargo Workspace 構造への移行](../adr/0002-cargo-workspace-structure.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)

## 方針

### アプローチ

1. workspace 構造の作成
2. shared パッケージの実装
3. server パッケージの実装
4. client パッケージの実装
5. import パスの修正
6. 統合テストの修正
7. テストと動作確認

### 設計方針

#### パッケージ構成

```txt
chat-app-rs/
├── Cargo.toml           # workspace root
├── tests/               # 統合テスト（workspace root に配置）
└── packages/
    ├── shared/          # 技術的ユーティリティ
    ├── server/          # Server（Layered Architecture）
    └── client/          # Client（シンプル構成）
```

#### 依存関係

```txt
shared
  ↑
  ├── server (depends on: shared)
  └── client (depends on: shared, server/infrastructure/dto)
```

### 品質基準

- 全てのテストが通ること
- cargo build --workspace が成功すること
- cargo clippy --workspace が警告なしで通ること
- cargo fmt --workspace が適用されていること
- server と client のバイナリが正常に起動すること

## タスク

### workspace 構造の作成

- [x] workspace root の Cargo.toml 作成
  - [x] `[workspace]` セクションの定義
  - [x] members の指定（packages/shared, packages/server, packages/client）
  - [x] workspace.dependencies の定義（共通依存関係）
  - [x] `[workspace.package]` で edition と version を統一
  - [x] テストの実施

- [x] packages/ ディレクトリの作成
  - [x] `mkdir packages` の実行

### shared パッケージの実装

- [x] 基本構造の作成
  - [x] `cargo new --lib packages/shared` の実行
  - [x] 依存関係の定義（packages/shared/Cargo.toml に chrono など追加）
  - [x] version.workspace = true, edition.workspace = true の設定

- [x] 既存コードの移動
  - [x] src/common/time.rs → packages/shared/src/time.rs
  - [x] src/common/logger.rs → packages/shared/src/logger.rs
  - [x] Clock trait, get_jst_timestamp, timestamp_to_jst_rfc3339 の移動
  - [x] テストコードの移動

- [x] テスト
  - [x] cargo test -p shared の実行
  - [x] 全テストが通ることを確認（7 passed）

### server パッケージの実装

- [x] 基本構造の作成
  - [x] `cargo new --lib packages/server` の実行
  - [x] 依存関係の定義（packages/server/Cargo.toml に axum, tokio, tracing など追加）
  - [x] shared パッケージへの依存を追加
  - [x] version.workspace = true, edition.workspace = true の設定

- [x] domain 層の移動
  - [x] src/domain/ → packages/server/src/domain/
  - [x] entity.rs, value_object.rs, repository.rs, message_pusher.rs, factory.rs, error.rs
  - [x] テストコードの移動

- [x] usecase 層の移動
  - [x] src/usecase/ → packages/server/src/usecase/
  - [x] connect_participant.rs, disconnect_participant.rs, send_message.rs
  - [x] get_rooms.rs, get_room_detail.rs, get_room_state.rs, error.rs
  - [x] テストコードの移動

- [x] infrastructure 層の移動
  - [x] src/infrastructure/ → packages/server/src/infrastructure/
  - [x] repository/, message_pusher/, dto/, conversion.rs
  - [x] テストコードの移動

- [x] ui 層の移動
  - [x] src/ui/ → packages/server/src/ui/
  - [x] handler/http.rs, handler/websocket.rs, state.rs, server.rs
  - [x] テストコードの移動

- [x] バイナリの作成
  - [x] packages/server/src/bin/server.rs 作成
  - [x] src/bin/server.rs の内容を移動
  - [x] エントリーポイントの調整

- [x] import パスの修正
  - [x] `crate::common::time` → `shared::time` への更新
  - [x] テストコード内の import も更新

- [x] テスト
  - [x] cargo test -p server の実行
  - [x] 全テストが通ることを確認（51 passed）

### client パッケージの実装

- [x] 基本構造の作成
  - [x] `cargo new --lib packages/client` の実行
  - [x] 依存関係の定義（packages/client/Cargo.toml に tokio-tungstenite など追加）
  - [x] shared パッケージへの依存を追加
  - [x] server/infrastructure/dto への依存を追加
  - [x] version.workspace = true, edition.workspace = true の設定

- [x] 既存コードの移動
  - [x] src/utils/client/domain.rs → packages/client/src/domain.rs
  - [x] src/utils/client/formatter.rs → packages/client/src/formatter.rs
  - [x] src/utils/client/session.rs → packages/client/src/session.rs
  - [x] src/utils/client/error.rs, runner.rs, ui.rs の移動
  - [x] テストコードの移動

- [x] バイナリの作成
  - [x] packages/client/src/bin/client.rs 作成
  - [x] src/bin/client.rs の内容を移動
  - [x] エントリーポイントの調整

- [x] import パスの修正
  - [x] `chat_app_rs::common::logger` → `shared::logger` への更新
  - [x] `crate::infrastructure::dto` → `server::infrastructure::dto` への更新
  - [x] `chat_app_rs::utils::client` → `client` への更新

- [x] テスト
  - [x] cargo test -p client の実行
  - [x] 全テストが通ることを確認

### import パスの修正

- [x] shared パッケージ内の import 修正
  - [x] 相互参照の確認
  - [x] doctestの修正（`chat_app_rs::` → `shared::`）
  - [x] テストの実行

- [x] server パッケージ内の import 修正
  - [x] `crate::common::time` → `shared::time`
  - [x] `crate::domain` は `crate::domain` のまま（パッケージ内）
  - [x] `crate::usecase` は `crate::usecase` のまま（パッケージ内）
  - [x] `crate::infrastructure` は `crate::infrastructure` のまま（パッケージ内）
  - [x] `crate::ui` は `crate::ui` のまま（パッケージ内）
  - [x] テストコード内の `crate::common::time` も修正
  - [x] テストの実行

- [x] client パッケージ内の import 修正
  - [x] `crate::common::time` → `shared::time`
  - [x] `crate::infrastructure::dto` → `server::infrastructure::dto`
  - [x] `chat_app_rs::utils::client` → `client`
  - [x] テストの実行

### 統合テストの修正

- [x] tests/ ディレクトリは workspace root に残す
  - [x] tests/fixtures/mod.rs の確認
  - [x] tests/http_api.rs, tests/websocket_connection.rs, tests/websocket_messaging.rs の確認

- [x] TestServer::start() の修正
  - [x] `cargo run --bin server` → `cargo run -p server --bin server`
  - [x] テストの実行

- [x] TestClient::start_with_delay() の修正
  - [x] `cargo run --bin client` → `cargo run -p client --bin client`
  - [x] テストの実行

- [x] 統合テストの実行
  - [x] cargo test --test http_api
  - [x] cargo test --test websocket_connection
  - [x] cargo test --test websocket_messaging
  - [x] 全テストが通ることを確認

### 全体テストと動作確認

- [x] workspace 全体のテスト
  - [x] cargo test --workspace の実行
  - [x] 全テストが通ることを確認（58 passed）

- [x] ビルドの確認
  - [x] cargo build --workspace の実行
  - [x] 全パッケージがビルドできることを確認

- [x] Clippy の実行
  - [x] cargo clippy --workspace --all-targets --all-features
  - [x] 警告がないことを確認

- [x] フォーマットの確認
  - [x] cargo fmt --workspace
  - [x] フォーマットが適用されることを確認

- [x] バイナリの起動確認
  - [x] cargo run -p server --bin server -- --help の実行
  - [x] cargo run -p client --bin client -- --help の実行
  - [x] 正常に起動することを確認

### 後処理

- [x] 不要なファイルの削除
  - [x] src/ ディレクトリの削除（移動完了後）
  - [x] 古い Cargo.toml のバックアップは保持（Cargo.toml.backup）

- [x] Cargo.toml の整理
  - [x] コメントの削除
  - [x] 依存関係のアルファベット順ソート

- [x] ドキュメントの更新
  - [x] README.md の更新（ビルド・実行コマンド）
  - [x] AGENTS.md の更新（プロジェクト構造）
  - [x] docs/documentations/software-architecture.md の更新

## 進捗状況

- **作成日**: 2025-11-13 18:12:06
- **開始日**: 2025-11-13 18:54:00
- **完了日**: 2025-11-13 20:15:00
- **現在のフェーズ**: 完了
- **完了タスク数**: 54 / 54
- **次のアクション**: なし（すべてのタスクが完了）
- **ブロッカー**: なし

## 備考

### 実装時の注意事項

1. **段階的な移行**
   - 一度に全てを移動せず、パッケージごとに移動してテストを実行
   - shared → server → client の順に実装
   - 各ステップでテストが通ることを確認

2. **import パスの方針**
   - パッケージ間の参照は `パッケージ名::` を使用（例: `shared::time`）
   - パッケージ内の参照は `crate::` を使用（例: `crate::domain`）
   - または `use server::domain` のように明示的にパッケージ名を使用

3. **client の DTO 依存**
   - client は server の infrastructure/dto に依存する
   - これは client が Rust で実装されているため、DTO を再利用している
   - TypeScript などで実装する場合は独自に DTO を定義する

4. **統合テストの配置**
   - tests/ ディレクトリは workspace root に残す
   - 実際のバイナリを起動してテストするため、パッケージ内に配置する必要がない

5. **Cargo.toml の管理**
   - workspace root の Cargo.toml で共通の依存関係を管理
   - 各パッケージの Cargo.toml では workspace.dependencies を参照
   - バージョン番号は workspace root で一元管理

### トラブルシューティング

1. **import エラーが発生した場合**
   - パッケージ名が正しいか確認
   - Cargo.toml の dependencies に追加されているか確認
   - cargo clean && cargo build で再ビルド

2. **テストが失敗する場合**
   - import パスが正しいか確認
   - 各パッケージのテストを個別に実行（cargo test -p <package>）
   - テストコードの import も修正が必要

3. **統合テストが失敗する場合**
   - TestServer::start() のコマンドが正しいか確認
   - server バイナリが正しくビルドされているか確認
   - ポート番号の衝突がないか確認

### Phase 2, Phase 3 への影響

- **Phase 2（ドメインモデル拡張）**
  - domain 層の拡張は packages/server/src/domain/ で実施
  - workspace 構造により、影響範囲が明確になる

- **Phase 3（TUI 実装）**
  - packages/tui-client/ を追加する形で実装
  - shared と server/infrastructure/dto に依存

## 参考資料

### Cargo Workspace

- [The Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Workspace Dependencies](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table)

### プロジェクト関連

- [ADR 0002: Cargo Workspace 構造への移行](../adr/0002-cargo-workspace-structure.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)
- [レイヤードアーキテクチャ基礎](../documentations/layered-architecture-basic.md)
