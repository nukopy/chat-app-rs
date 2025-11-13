# Phase 1: ドメインモデリング

## 概要

### 目的

DDD の原則に従ったコンテキストマップとドメインモデル図を作成し、複数ルーム対応チャットアプリケーションのドメイン設計を明確化する。

### 背景

- 現状は単一ルームのシンプルなチャットアプリケーション
- 複数ルーム、ユーザー管理、ルーム認証機能を追加するための設計が必要
- DDD の成果物（コンテキストマップ、ドメインモデル図）を作成することで、実装の指針を明確にする

### スコープ

- コンテキストマップの作成（Mermaid 形式）
- ドメインモデル図の作成（Mermaid 形式）
- ドキュメントへの追加

**スコープ外**:

- ドメインモデルの実装（Phase 2 で実施）

### 参照

- [DDD スタイルガイド](../documentations/ddd.md)
- [次フェーズ: Phase 2 バックエンド拡張](./20251112-061030_phase2-backend-extension.md)

## 方針

### アプローチ

1. コンテキストマップの作成
2. ドメインモデル図の作成
3. ドキュメント化

### 設計方針

#### ドメインモデル

**User（ユーザー）**

- アプリケーションのユーザーを表すエンティティ
- `user_id`: ユーザー ID（Value Object）
- `username`: ユーザー名（Value Object）

**Room（ルーム）**

- チャットルームを表す集約ルート
- `room_id`: ルーム ID（Value Object）
- `room_name`: ルーム名（Value Object、50 文字以内）
- `password`: ルームパスワード（Value Object、6 文字）
- `participants`: 参加者リスト
- `messages`: メッセージ履歴
- `created_at`: 作成日時（Value Object）
- `last_message_at`: 最終メッセージ日時（Value Object）

**Participant（参加者）**

- User が特定の Room に入室したときに作成されるエンティティ
- `participant_id`: 参加者 ID（Value Object）
- `user_id`: ユーザー ID（Value Object）
- `room_id`: ルーム ID（Value Object）
- `client_id`: WebSocket クライアント ID（Value Object）
- `connected_at`: 接続日時（Value Object）

**ChatMessage（チャットメッセージ）**

- `message_id`: メッセージ ID（Value Object）
- `room_id`: ルーム ID（Value Object）
- `from_participant_id`: 送信者の参加者 ID（Value Object）
- `content`: メッセージ内容（Value Object）
- `timestamp`: 送信日時（Value Object）

### 品質基準

- DDD の原則に従った設計
- Mermaid 形式で図を作成（バージョン管理可能）
- 図の可読性を確保

## タスク

### コンテキストマップの作成

- [ ] コンテキストの洗い出し
  - [ ] 「チャット管理コンテキスト」の定義
  - [ ] 「ユーザー管理コンテキスト」の定義
  - [ ] 「ルーム管理コンテキスト」の定義
  - [ ] コンテキスト間の依存関係の整理

- [ ] Mermaid 形式でコンテキストマップを作成
  - [ ] コンテキストをクラス図形式で表現
  - [ ] コンテキスト間の依存関係を矢印で表現

- [ ] ドキュメントに追加
  - [ ] `docs/documentations/context-map.md` を作成
  - [ ] コンテキストマップの図を追加
  - [ ] 各コンテキストの説明を追加

### ドメインモデル図の作成

- [ ] User エンティティの定義
  - [ ] User の属性を洗い出し
  - [ ] User の責務を定義
  - [ ] User の不変条件を定義

- [ ] Room 集約の定義
  - [ ] Room の属性を洗い出し
  - [ ] Room の責務を定義
  - [ ] Room の不変条件を定義（容量制限、パスワードルールなど）

- [ ] Participant エンティティの定義
  - [ ] Participant の属性を洗い出し
  - [ ] Participant の責務を定義
  - [ ] User と Room の関連を明確化

- [ ] ChatMessage エンティティの定義
  - [ ] ChatMessage の属性を洗い出し
  - [ ] ChatMessage の責務を定義
  - [ ] Room との関連を明確化

- [ ] Repository の定義
  - [ ] UserRepository の責務を定義
  - [ ] RoomRepository の責務を定義
  - [ ] 各 Repository のメソッドシグネチャを定義

- [ ] 各エンティティの関係性を図示
  - [ ] User と Participant の関係（1:N）
  - [ ] Room と Participant の関係（1:N）
  - [ ] Room と ChatMessage の関係（1:N）
  - [ ] Participant と ChatMessage の関係（1:N）

- [ ] Mermaid 形式でドメインモデル図を作成
  - [ ] クラス図形式でエンティティを表現
  - [ ] 属性とメソッドを記載
  - [ ] 依存関係を矢印で表現

- [ ] ドキュメントに追加
  - [ ] `docs/documentations/domain-model.md` を作成
  - [ ] ドメインモデル図を追加
  - [ ] 各エンティティの詳細説明を追加
  - [ ] Value Object の定義を追加

## 進捗状況

- **作成日**: 2025-11-12 06:10:30
- **開始日**: 未定
- **現在のフェーズ**: Phase 1 - ドメインモデリング
- **完了タスク数**: 0 / 23
- **次のアクション**: コンテキストの洗い出し
- **ブロッカー**: なし

## 備考

### 実装時の注意事項

1. **図の一貫性**
   - Mermaid の記法を統一する
   - 矢印の方向（依存の向き）を明確にする

2. **ドキュメントの配置**
   - `docs/documentations/` 配下に配置
   - README からリンクを追加

3. **レビュー**
   - Phase 1 完了後、設計をレビュー
   - Phase 2 の実装前に設計の妥当性を確認

### 次フェーズへの引き継ぎ

- Phase 1 完了後、Phase 2 でドメインモデルを実装
- Phase 1 の成果物（コンテキストマップ、ドメインモデル図）を Phase 2 の実装指針とする

## 参考資料

- [DDD スタイルガイド](../documentations/ddd.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)
- [Mermaid Class Diagrams](https://mermaid.js.org/syntax/classDiagram.html)
