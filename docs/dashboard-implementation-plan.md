# 学友会率計算結果表示ページ実装計画

## 概要
ダッシュボードページから遷移する、学友会率計算結果表示ページを実装する。
単一のExcelファイルおよび複数のExcelファイルの学友会率計算結果を表示し、
結果はタブで切り替え可能なテーブル形式で表示する。エラーファイルは別セクションでリスト表示する。

## ページ遷移
- ダッシュボードページ（/dashboard）の「Excelファイルを選択」ボタンから単一ファイル計算ページ（/gakuyukai-rates）に遷移
- ダッシュボードページ（/dashboard）の「フォルダを選択」ボタンから複数ファイル計算ページ（/gakuyukai-rates）に遷移
- 遷移時にクエリパラメータ`mode=single`または`mode=multiple`を付与し、初期表示タブを制御

## データ構造

### 単一ファイル用（CircleInfo）
```typescript
interface CircleInfo {
  name: string;
  file_path: string;
  member_count: number;
  gakuyukai_member_count: number;
  rate_string: string;
}
```

### 複数ファイル用（CircleGakuyukaiRates）
```typescript
interface CircleGakuyukaiRates {
  circles: CircleInfo[];
  error_file_path: string[];
}
```

## UI構成

### 1. タブコンテナ
- 単一ファイル表示タブ
- 複数ファイル表示タブ

### 2. 単一ファイルタブの内容
- ファイル選択ボタン
- 結果テーブル
  - 列：
    - 団体名
    - ファイルパス
    - 総メンバー数
    - 学友会メンバー数
    - 学友会率

### 3. 複数ファイルタブの内容
- フォルダ選択ボタン
- 結果テーブル（単一ファイルと同じ列構成）
- エラーファイルセクション
  - エラーファイルのパスをリスト表示

## 状態管理
```typescript
let activeTab: 'single' | 'multiple' = 'single';
let singleFileResult: CircleInfo | null = null;
let multipleFileResults: CircleGakuyukaiRates | null = null;
let loading: boolean = false;
```

## 実装手順

1. ページ構造の実装
   - 新規ページ（/gakuyukai-rates）の作成
   - クエリパラメータによる初期タブ制御の実装

2. ダッシュボードの修正
   - 「Excelファイルを選択」ボタンのリンク実装
   - 「フォルダを選択」ボタンのリンク実装

3. 結果表示ページの実装
   - タブコンポーネントの作成
   - 各タブのコンテナ作成
   - ファイル選択ボタンの実装
   - 結果表示テーブルの実装
   - エラーファイルリストの実装
   - Rust関数との連携

4. スタイリング
   - テーブルのスタイリング
   - タブのスタイリング
   - ボタンのスタイリング
   - エラーセクションのスタイリング

## 注意事項
- ソート機能は実装しない
- ページネーションは実装しない
- エラーファイルは単純なリスト表示とする