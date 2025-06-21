import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

// カスタムメトリクス
const errorRate = new Rate('errors');
const responseTime = new Trend('response_time');

// 環境変数からRPS（Requests Per Second）を取得、デフォルトは20
const RPS = parseInt(__ENV.RPS) || 20;
const BASE_URL = __ENV.BASE_URL || 'http://localhost:4000'; // Rustバックエンドのデフォルトポート

// RPS に基づいてVU数と反復レートを計算
// RPS = VUs * iterations_per_second_per_VU
// 各VUが1秒間に実行する反復回数を1とすると、VU数 = RPS
const VUS = Math.min(RPS, 500); // 最大500 VU
const DURATION = '1m'; // テスト実行時間

export const options = {
  scenarios: {
    constant_request_rate: {
      executor: 'constant-arrival-rate',
      rate: RPS, // 1秒あたりのリクエスト数
      timeUnit: '1s',
      duration: DURATION,
      preAllocatedVUs: Math.min(VUS, 50), // 事前割り当てVU数
      maxVUs: VUS, // 最大VU数
    },
  },
  thresholds: {
    http_req_duration: ['p(95)<1000'], // 95%のリクエストが1秒以内
    http_req_failed: ['rate<0.1'], // エラー率10%以下
    errors: ['rate<0.1'], // カスタムエラー率10%以下
  },
};

// リクエストオプション
const requestOptions = {
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
  timeout: '30s',
};

export function setup() {
  console.log(`🚀 Starting stress test with ${RPS} requests/second`);
  console.log(`📊 Test configuration:`);
  console.log(`   - Target RPS: ${RPS}`);
  console.log(`   - Max VUs: ${VUS}`);
  console.log(`   - Duration: ${DURATION}`);
  console.log(`   - Base URL: ${BASE_URL}`);
}

export default function () {
  // ランダムにエンドポイントを選択してテスト
  const endpoints = [
    testGetProducts,
  ];
  
  const randomEndpoint = endpoints[Math.floor(Math.random() * endpoints.length)];
  randomEndpoint();
}

function testGetProducts() {
  const response = http.get(`${BASE_URL}/products`, requestOptions);
  
  const success = check(response, {
    'Get Products: status is 2xx': (r) => r.status >= 200 && r.status < 300,
    'Get Products: response time < 500ms': (r) => r.timings.duration < 500,
    'Get Products: has body': (r) => r.body && r.body.length > 0,
  });
  
  recordMetrics(response, success, 'GetProducts');
}

function recordMetrics(response, success, operation) {
  // カスタムメトリクスを記録
  errorRate.add(!success);
  responseTime.add(response.timings.duration);
  
  // デバッグ情報（高負荷時は出力を制限）
  if (!success && Math.random() < 0.1) { // 10%の確率でエラーログ出力
    console.log(`❌ ${operation} failed: Status ${response.status}, Duration ${response.timings.duration}ms`);
  }
}

export function teardown(data) {
  console.log(`✅ Stress test completed with ${RPS} requests/second`);
  console.log(`📈 Check the results for performance metrics and error rates`);
} 