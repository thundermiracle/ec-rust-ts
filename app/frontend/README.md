# EC Frontend

このプロジェクトは[Next.js](https://nextjs.org/)で構築されたECサイトのフロントエンドです。

## 🚀 開発環境のセットアップ

```bash
pnpm install
pnpm dev
```

## 📚 API自動生成

このプロジェクトでは、バックエンドのOpenAPI仕様から自動的にRTK Query APIコードが生成されます。

### APIコードの再生成

```bash
# 手動でAPIコードを生成
pnpm generate-api

# 開発サーバー起動時に自動生成
pnpm dev
```

### API生成の仕組み

1. **OpenAPI仕様**: `http://localhost:4000/api-docs/openapi.json`からスキーマを取得
2. **ベースAPI**: `src/lib/baseApi.ts`で基本設定を定義
3. **分割されたAPI**: `src/lib/generatedApi/`に機能別の自動生成ファイル
   - `products.ts` - 商品関連のAPI
   - `categories.ts` - カテゴリ関連のAPI  
   - `index.ts` - 統合されたAPIエクスポート
4. **API再エクスポート**: `src/lib/api.ts`で使いやすい形で再エクスポート

### 利用可能なAPI Hooks

生成されたAPIでは以下のhooksが利用できます：

```typescript
// 統合されたAPIから（推奨）
import {
  useGetCategoryListQuery,
  useGetProductListQuery,
  useGetProductQuery,
} from '@/lib/api'

// または、機能別に個別インポート
import { useGetProductListQuery, useGetProductQuery } from '@/lib/api'
import { useGetCategoryListQuery } from '@/lib/api'

// カテゴリリスト取得
const { data: categories } = useGetCategoryListQuery()

// 商品リスト取得
const { data: products } = useGetProductListQuery()

// 商品詳細取得
const { data: product } = useGetProductQuery({ id: 'product-123' })
```

### 分割されたAPI構造

機能別にAPIが分割されており、必要に応じて個別にインポート可能：

```typescript
// 商品関連のみを使用
import { productsApi, useGetProductQuery } from '@/lib/api'

// カテゴリ関連のみを使用  
import { categoriesApi, useGetCategoryListQuery } from '@/lib/api'
```

## 🛠 技術スタック

- **フレームワーク**: Next.js 15
- **UI**: React 19 + Tailwind CSS
- **状態管理**: Redux Toolkit + RTK Query
- **API生成**: @rtk-query/codegen-openapi
- **TypeScript**: 型安全な開発環境

## 📁 プロジェクト構造

```
src/
├── app/                 # Next.js App Router
├── components/          # 再利用可能なコンポーネント
├── lib/
│   ├── api.ts          # API hooks の再エクスポート
│   ├── baseApi.ts      # RTK Query ベース設定
│   ├── generatedApi/   # 分割された自動生成API（編集不可）
│   │   ├── index.ts    # 統合されたAPIエクスポート
│   │   ├── products.ts # 商品関連API
│   │   └── categories.ts # カテゴリ関連API
│   └── store.ts        # Redux store 設定
└── types/              # TypeScript 型定義
```

## ⚠️ 重要な注意事項

- `src/lib/generatedApi/`ディレクトリ内のファイルは自動生成されます。直接編集しないでください。
- バックエンドのOpenAPI仕様を変更した場合は、`pnpm generate-api`を実行してください。
- 開発サーバー起動時（`pnpm dev`）には自動的にAPI生成が実行されます。
- API生成時に一時的に単一ファイルが作成され、その後自動的に機能別に分割されます。

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
