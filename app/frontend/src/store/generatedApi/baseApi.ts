import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'

// APIエンドポイントのベースURL
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:4000'

// ベースAPIの定義（OpenAPI codegen が拡張する）
export const api = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({
    baseUrl: API_BASE_URL,
    prepareHeaders: (headers) => {
      headers.set('Content-Type', 'application/json')
      return headers
    },
  }),
  tagTypes: ['Product', 'Category', 'PaymentMethods'], // OpenAPI仕様から自動推論される
  endpoints: () => ({}), // OpenAPI codegenで自動生成
}) 