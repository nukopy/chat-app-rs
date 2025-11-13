# Phase 2: バックエンド拡張

## 概要

### 目的

Phase 1 で設計したドメインモデルを実装し、複数ルーム対応、ユーザー管理、ルーム認証機能を追加する。

### 背景

- Phase 1 でドメインモデルの設計が完了
- 現状は単一ルームのシンプルなチャットアプリケーション
- 複数ルーム、ユーザー管理、ルーム認証機能を実装する必要がある

### スコープ

- ドメインモデルの実装（Value Object, エンティティ）
- 複数ルーム対応の実装
- ユーザー管理機能の追加
- ルーム認証機能の追加
- メッセージ配信ロジックの変更（Room 単位）
- API エンドポイントの追加

**スコープ外**:

- TUI 実装（Phase 3 で実施）

### 参照

- [前フェーズ: Phase 1 ドメインモデリング](./20251112-061030_phase1-domain-modeling.md)
- [次フェーズ: Phase 3 TUI 実装](./20251112-061030_phase3-tui-implementation.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)

## 方針

### アプローチ

1. ドメインモデルの実装
2. Repository の拡張
3. UseCase の実装
4. API エンドポイントの追加
5. テストの実装

### 設計方針

#### レイヤー構成

- **Domain 層**: Value Object, エンティティ, Repository trait
- **Infrastructure 層**: Repository 実装, MessagePusher 拡張
- **UseCase 層**: ビジネスロジックの実装
- **UI 層**: HTTP API エンドポイント

#### メッセージ配信ロジック

現状:

```
client_id のみで push 対象を決定
→ 全クライアントから送信者を除外
```

変更後:

```
room_id と client_id で push 対象を決定
→ 特定 Room の参加者から送信者を除外
```

### 品質基準

- 全ての変更に対してユニットテストを実装
- DDD の原則に従った実装
- Clean Architecture の層分離を維持
- 全てのドメインモデルは Value Object として実装
- 既存機能との互換性を維持（段階的な移行）

## タスク

### ドメインモデルの実装

#### User エンティティの実装

- [ ] `UserId` Value Object の実装
  - [ ] フォーマット検証（UUID v4）
  - [ ] テストの実装

- [ ] `Username` Value Object の実装
  - [ ] 長さ制限（1-50 文字）
  - [ ] 禁止文字のチェック
  - [ ] テストの実装

- [ ] `User` エンティティの実装
  - [ ] 構造体の定義
  - [ ] コンストラクタの実装
  - [ ] テストの実装

#### Room 集約の拡張

- [ ] `RoomName` Value Object の実装
  - [ ] 長さ制限（1-50 文字）
  - [ ] テストの実装

- [ ] `RoomPassword` Value Object の実装
  - [ ] 長さ制限（6 文字固定）
  - [ ] パスワード検証メソッド
  - [ ] テストの実装

- [ ] `LastMessageAt` Value Object の実装
  - [ ] Timestamp をラップ
  - [ ] テストの実装

- [ ] Room エンティティの拡張
  - [ ] `room_name` フィールドの追加
  - [ ] `password` フィールドの追加
  - [ ] `last_message_at` フィールドの追加
  - [ ] パスワード検証メソッドの実装
  - [ ] テストの実装

#### Participant エンティティの再設計

- [ ] `ParticipantId` Value Object の実装
  - [ ] フォーマット検証（UUID v4）
  - [ ] テストの実装

- [ ] Participant エンティティの再実装
  - [ ] `participant_id` フィールドの追加
  - [ ] `user_id` フィールドの追加
  - [ ] `room_id` フィールドの追加
  - [ ] `client_id` フィールドの維持
  - [ ] User と Room の関連を持つように変更
  - [ ] テストの実装

#### ChatMessage エンティティの拡張

- [ ] `MessageId` Value Object の実装
  - [ ] フォーマット検証（UUID v4）
  - [ ] テストの実装

- [ ] ChatMessage エンティティの拡張
  - [ ] `message_id` フィールドの追加
  - [ ] `room_id` フィールドの追加
  - [ ] `from_participant_id` フィールドの追加
  - [ ] テストの実装

### Repository の拡張

- [ ] RoomRepository の拡張
  - [ ] 複数ルーム管理への対応
  - [ ] `create_room` メソッドの追加
  - [ ] `get_room_by_id` メソッドの追加
  - [ ] `get_all_rooms` メソッドの追加
  - [ ] `update_last_message_at` メソッドの追加
  - [ ] InMemoryRoomRepository の実装更新
  - [ ] テストの実装

- [ ] UserRepository の実装
  - [ ] Repository trait の定義
  - [ ] `create_user` メソッドの定義
  - [ ] `get_user_by_id` メソッドの定義
  - [ ] `get_user_by_username` メソッドの定義
  - [ ] InMemoryUserRepository の実装
  - [ ] テストの実装

### UseCase の実装

#### ルーム管理 UseCase

- [ ] CreateRoomUseCase の実装
  - [ ] ルーム作成ロジック
  - [ ] パスワードの設定
  - [ ] テストの実装

- [ ] GetRoomsUseCase の拡張
  - [ ] 複数ルームの一覧取得
  - [ ] ソート（最終更新日時順）
  - [ ] テストの実装

- [ ] GetRoomDetailUseCase の拡張
  - [ ] ルーム詳細情報の取得
  - [ ] 参加者数の計算
  - [ ] テストの実装

- [ ] VerifyRoomPasswordUseCase の実装
  - [ ] パスワード検証ロジック
  - [ ] テストの実装

#### ユーザー管理 UseCase

- [ ] RegisterUserUseCase の実装
  - [ ] ユーザー登録ロジック
  - [ ] ユーザー名重複チェック
  - [ ] テストの実装

- [ ] AuthenticateUserUseCase の実装
  - [ ] ユーザー認証ロジック
  - [ ] テストの実装

#### 参加者管理 UseCase

- [ ] JoinRoomUseCase の実装
  - [ ] パスワード認証を含む入室処理
  - [ ] Participant の作成
  - [ ] テストの実装

- [ ] ConnectParticipantUseCase の拡張
  - [ ] Room ID を考慮した接続処理
  - [ ] テストの実装

#### メッセージ管理 UseCase

- [ ] SendMessageUseCase の拡張
  - [ ] Room 単位でのメッセージ配信
  - [ ] ブロードキャスト対象の選定（Room の参加者から送信者を除外）
  - [ ] `last_message_at` の更新
  - [ ] テストの実装

### Infrastructure 層の拡張

- [ ] MessagePusher の拡張
  - [ ] Room ID を考慮したメッセージ配信
  - [ ] `broadcast_to_room` メソッドの追加
  - [ ] テストの実装

### API エンドポイントの追加

- [ ] ユーザー管理 API
  - [ ] `POST /api/users` - ユーザー登録
  - [ ] HTTP ハンドラの実装
  - [ ] テストの実装

- [ ] ルーム管理 API
  - [ ] `POST /api/rooms` - ルーム作成
  - [ ] `GET /api/rooms` - ルーム一覧取得
  - [ ] `GET /api/rooms/:room_id` - ルーム詳細取得
  - [ ] `POST /api/rooms/:room_id/auth` - ルーム認証
  - [ ] HTTP ハンドラの実装
  - [ ] テストの実装

- [ ] 参加者管理 API
  - [ ] `POST /api/rooms/:room_id/participants` - ルーム入室
  - [ ] HTTP ハンドラの実装
  - [ ] テストの実装

### WebSocket の拡張

- [ ] WebSocket 接続時の Room ID 指定
  - [ ] クエリパラメータに `room_id` を追加
  - [ ] 接続ハンドラの更新
  - [ ] テストの実装

- [ ] メッセージ送信時の Room ID 検証
  - [ ] 送信者が Room に参加しているか検証
  - [ ] テストの実装

### 統合テスト

- [ ] 複数ルーム対応の統合テスト
  - [ ] 複数ルームの作成
  - [ ] 異なるルームへの参加
  - [ ] ルーム間のメッセージ隔離の検証

- [ ] ユーザー認証の統合テスト
  - [ ] ユーザー登録
  - [ ] ユーザー認証
  - [ ] 認証失敗のテスト

- [ ] ルーム認証の統合テスト
  - [ ] 正しいパスワードでの入室
  - [ ] 誤ったパスワードでの入室拒否

## 進捗状況

- **作成日**: 2025-11-12 06:10:30
- **開始日**: 未定（Phase 1 完了後）
- **現在のフェーズ**: Phase 2 - バックエンド拡張
- **完了タスク数**: 0 / 62
- **次のアクション**: Phase 1 の完了を待つ
- **ブロッカー**: Phase 1 の完了

## 備考

### 実装時の注意事項

1. **段階的な実装**
   - 既存機能を壊さないように、段階的に実装する
   - フィーチャーフラグを使用した段階的な移行を検討

2. **互換性の維持**
   - Phase 2 完了時点で、既存の CLI クライアントも動作するようにする
   - 単一ルームモードと複数ルームモードの共存

3. **テストの充実**
   - ドメインモデルの変更に伴い、既存のテストを更新
   - 新機能には必ずユニットテストと統合テストを実装

4. **ドキュメントの更新**
   - API 仕様書を作成・更新
   - README の更新（新機能の説明）

### パフォーマンス考慮事項

- 複数ルーム対応により、メモリ使用量が増加する可能性がある
- ルーム数が増えた場合の検索・一覧取得のパフォーマンスに注意
- 将来的にはページネーションの実装を検討

### 次フェーズへの引き継ぎ

- Phase 2 完了後、Phase 3 で TUI を実装
- Phase 2 で追加した API エンドポイントを Phase 3 の TUI から利用

## 参考資料

- [DDD スタイルガイド](../documentations/ddd.md)
- [ソフトウェアアーキテクチャ](../documentations/software-architecture.md)
- [レイヤードアーキテクチャ基礎](../documentations/layered-architecture-basic.md)
- [Phase 1 ドメインモデリング](./20251112-061030_phase1-domain-modeling.md)
