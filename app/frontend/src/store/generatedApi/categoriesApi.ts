import { api } from "./baseApi";
export const addTagTypes = ["Categories"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      getCategoryList: build.query<
        GetCategoryListApiResponse,
        GetCategoryListApiArg
      >({
        query: () => ({ url: `/categories` }),
        providesTags: ["Categories"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type GetCategoryListApiResponse =
  /** status 200 カテゴリリスト取得成功 */ GetCategoryListResponse;
export type GetCategoryListApiArg = void;
export type CategoryResponse = {
  /** 表示順序 */
  displayOrder: number;
  /** カテゴリID */
  id: string;
  /** カテゴリ名 */
  name: string;
  /** 親カテゴリID（階層構造の場合） */
  parentId?: string;
  /** カテゴリスラッグ */
  slug: string;
};
export type GetCategoryListResponse = {
  /** カテゴリ一覧 */
  categories: CategoryResponse[];
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export const { useGetCategoryListQuery } = injectedRtkApi;
