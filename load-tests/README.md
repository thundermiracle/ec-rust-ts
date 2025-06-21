# Load Tests

バックエンドAPI用のk6ストレステストです。

## 前提条件

- [k6](https://k6.io/docs/get-started/installation/)がインストール済み
- バックエンドサーバーが起動済み（デフォルト: http://localhost:4000）

## 使用方法

### Nxコマンドで実行

```bash
# 20 requests/second
nx stress:light load-tests

# 50 requests/second  
nx stress:medium load-tests

# 100 requests/second
nx stress:high load-tests

# 500 requests/second
nx stress:heavy load-tests

# 1000 requests/second
nx stress:extreme load-tests
```

### 直接k6で実行

```bash
cd load-tests

# 20 requests/second
k6 run --env RPS=20 src/stress-test.js

# 50 requests/second
k6 run --env RPS=50 src/stress-test.js

# 100 requests/second
k6 run --env RPS=100 src/stress-test.js

# 500 requests/second
k6 run --env RPS=500 src/stress-test.js

# 1000 requests/second
k6 run --env RPS=1000 src/stress-test.js
```

### カスタム設定

```bash
# カスタムURL指定
k6 run --env RPS=100 --env BASE_URL=http://localhost:4000 src/stress-test.js

# 結果をJSONで出力
k6 run --env RPS=100 --out json=results.json src/stress-test.js
```

## テスト対象エンドポイント

- `GET /products` - 商品一覧取得
- `GET /products/{id}` - 商品詳細取得

## メトリクス

- **Response Time**: レスポンス時間（95%ile < 1000ms）
- **Error Rate**: エラー率（< 10%）
- **Throughput**: スループット（指定したRPS）

## 結果の見方

テスト実行後、以下の情報が表示されます：

- `http_req_duration`: レスポンス時間の統計
- `http_req_failed`: 失敗率
- `http_reqs`: 総リクエスト数とRPS
- `iterations`: 実行された反復回数 