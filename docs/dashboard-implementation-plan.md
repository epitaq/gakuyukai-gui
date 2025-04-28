# 学友会率計算結果表示ページ実装計画

## 概要
単一のExcelファイルおよび複数のExcelファイルの学友会率計算結果を表示するページを実装する。
結果はタブで切り替え可能なテーブル形式で表示し、エラーファイルは別セクションでリスト表示する。

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

1. 基本構造の実装
   - タブコンポーネントの作成
   - 各タブのコンテナ作成

2. 単一ファイル機能の実装
   - ファイル選択ボタンの実装
   - 結果表示テーブルの実装
   - Rust関数との連携

3. 複数ファイル機能の実装
   - フォルダ選択ボタンの実装
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