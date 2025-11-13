# ドメインモデリングと TUI 実装（プロジェクト概要）

## 概要

現在のチャットアプリケーションを、複数ルーム対応のリッチな TUI アプリケーションに進化させるプロジェクト。

3 つのフェーズに分けて実装を進める：

1. **Phase 1: ドメインモデリング** - DDD の原則に従った設計
2. **Phase 2: バックエンド拡張** - ドメインモデルの実装と機能追加
3. **Phase 3: TUI 実装** - ratatui を使用した UI 実装

## フェーズドキュメント

### [Phase 1: ドメインモデリング](./20251112-061030_phase1-domain-modeling.md)

**目的**: コンテキストマップとドメインモデル図の作成

**主なタスク**:

- コンテキストマップの作成（Mermaid 形式）
- ドメインモデル図の作成（User, Room, Participant, ChatMessage）
- ドキュメント化

**タスク数**: 23

### [Phase 2: バックエンド拡張](./20251112-061030_phase2-backend-extension.md)

**目的**: ドメインモデルの実装と複数ルーム対応

**主なタスク**:

- ドメインモデルの実装（Value Object, エンティティ）
- 複数ルーム対応の実装
- ユーザー管理機能の追加
- ルーム認証機能の追加（6 文字パスワード）
- メッセージ配信ロジックの変更（Room 単位）
- API エンドポイントの追加

**タスク数**: 62

### [Phase 3: TUI 実装](./20251112-061030_phase3-tui-implementation.md)

**目的**: ratatui を使用したターミナルベースの UI 実装

**主なタスク**:

- ratatui のセットアップ
- 4 つの画面実装（ルーム一覧、ルーム詳細、パスワード入力、チャット）
- 画面遷移の実装
- キーボード操作の実装
- API/WebSocket 連携

**タスク数**: 55

## 全体進捗

- **作成日**: 2025-11-12 05:59:02
- **総タスク数**: 140 (23 + 62 + 55)
- **現在のフェーズ**: Phase 0 - 計画中
- **完了タスク数**: 0 / 140
- **次のアクション**: Phase 1 の開始

## ドメインモデル概要

### User（ユーザー）

- アプリケーションのユーザー
- `user_id`, `username`

### Room（ルーム）

- チャットルーム（集約ルート）
- `room_id`, `room_name`, `password`（6 文字）, `participants`, `messages`, `created_at`, `last_message_at`

### Participant（参加者）

- User が Room に入室したときに作成
- `participant_id`, `user_id`, `room_id`, `client_id`, `connected_at`

### ChatMessage（チャットメッセージ）

- Room 単位で管理
- `message_id`, `room_id`, `from_participant_id`, `content`, `timestamp`

## TUI 画面構成

1. **ルーム一覧画面**（トップ）
   - 既存ルームのリスト表示
   - 各ルームの情報：room_name, 参加者数, 最終更新日時, created_at

2. **ルーム詳細画面**
   - 選択したルームの詳細情報表示
   - 「一覧へ戻る」、「入室する」

3. **パスワード入力画面**
   - 6 文字のパスワード入力フォーム
   - 認証成功でチャット画面へ

4. **チャット画面**
   - メッセージ一覧、入力、参加者一覧

## 参考資料

### DDD 関連

- [DDD スタイルガイド](../documentations/ddd.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)
- [レイヤードアーキテクチャ基礎](../documentations/layered-architecture-basic.md)

### TUI 関連

- [ratatui Documentation](https://ratatui.rs/)
- [ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [crossterm Documentation](https://docs.rs/crossterm/)

## 関連 ADR（将来作成予定）

- ADR: 複数ルーム対応の設計判断
- ADR: ユーザー認証方式の選択
- ADR: TUI フレームワークの選定理由
