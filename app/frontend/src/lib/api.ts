import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'
import type { Product, ProductList } from '../types/product'

// APIエンドポイントのベースURL（バックエンドのURLに合わせて調整してください）
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:4000'

// 基本的なAPIスライスの定義
export const api = createApi({
  reducerPath: 'api',
  baseQuery: fetchBaseQuery({
    baseUrl: API_BASE_URL,
    prepareHeaders: (headers) => {
      // 必要に応じて認証ヘッダーなどを追加
      headers.set('Content-Type', 'application/json')
      return headers
    },
  }),
  tagTypes: ['Product', 'User'], // キャッシュタグの定義
  endpoints: (builder) => ({
    // 商品関連のエンドポイント例
    getProducts: builder.query<ProductList, void>({
      query: () => '/products',
      providesTags: ['Product'],
    }),
    getProduct: builder.query<Product, string>({
      query: (id) => `/products/${id}`,
      providesTags: (result, error, id) => [{ type: 'Product', id }],
    }),
  }),
})

// 自動生成されたフックをエクスポート
export const {
  useGetProductsQuery,
  useGetProductQuery,
} = api

 