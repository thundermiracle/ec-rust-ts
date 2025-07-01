import { api } from "./baseApi";
export const addTagTypes = ["Products"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      getProductList: build.query<
        GetProductListApiResponse,
        GetProductListApiArg
      >({
        query: () => ({ url: `/products` }),
        providesTags: ["Products"],
      }),
      getProduct: build.query<GetProductApiResponse, GetProductApiArg>({
        query: (queryArg) => ({ url: `/products/${queryArg.id}` }),
        providesTags: ["Products"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type GetProductListApiResponse =
  /** status 200 商品リスト取得成功 */ GetProductListResponse;
export type GetProductListApiArg = void;
export type GetProductApiResponse =
  /** status 200 商品詳細の取得成功 */ GetProductResponse;
export type GetProductApiArg = {
  /** 商品ID */
  id: string;
};
export type GetProductListItemResponse = {
  /** カテゴリー名 */
  category: string;
  /** 利用可能な色一覧 */
  colors: string[];
  /** 商品ID */
  id: string;
  /** 商品画像URL */
  image: string;
  /** ベストセラー商品かどうか */
  isBestSeller?: boolean;
  /** セール中かどうか */
  isOnSale?: boolean;
  /** 即配送可能かどうか */
  isQuickShip?: boolean;
  /** 売り切れかどうか */
  isSoldOut?: boolean;
  /** 商品名 */
  name: string;
  /** 基本価格（円） */
  price: number;
  /** セール価格（円） */
  salePrice?: number;
};
export type GetProductListResponse = {
  /** 次のページがあるかどうか */
  hasNextPage: boolean;
  /** 前のページがあるかどうか */
  hasPreviousPage: boolean;
  /** 現在のページ番号 */
  page: number;
  /** 1ページあたりの件数 */
  perPage: number;
  /** 商品一覧 */
  products: GetProductListItemResponse[];
  /** 総件数 */
  totalCount: number;
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export type VariantResponse = {
  /** 色 */
  color: string;
  /** サイズ */
  dimensions: string;
  /** 表示順 */
  displayOrder: number;
  /** バリエーションID */
  id: string;
  /** バリエーション画像URL */
  image?: string;
  /** セール中かどうか */
  isOnSale: boolean;
  /** 品切れかどうか */
  isSoldOut: boolean;
  /** 素材 */
  material: string;
  /** バリエーション名 */
  name: string;
  /** 価格（円） */
  price: number;
  /** セール価格（円） */
  salePrice?: number;
  /** SKU商品コード */
  skuCode: string;
};
export type GetProductResponse = {
  /** カテゴリー名 */
  category: string;
  /** 商品説明 */
  description: string;
  /** 商品ID */
  id: string;
  /** 商品画像URL一覧 */
  images: string[];
  /** ベストセラー商品かどうか */
  isBestSeller: boolean;
  /** 即配送可能かどうか */
  isQuickShip: boolean;
  /** 商品名 */
  name: string;
  /** バリエーション一覧 */
  variants?: VariantResponse[];
};
export const { useGetProductListQuery, useGetProductQuery } = injectedRtkApi;
