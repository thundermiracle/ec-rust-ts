import { api } from "./baseApi";
export const addTagTypes = ["Shipping"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      getShippingMethodList: build.query<
        GetShippingMethodListApiResponse,
        GetShippingMethodListApiArg
      >({
        query: () => ({ url: `/shipping-methods` }),
        providesTags: ["Shipping"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type GetShippingMethodListApiResponse =
  /** status 200 配送方法一覧の取得成功 */ GetShippingMethodListResponse;
export type GetShippingMethodListApiArg = void;
export type GetShippingMethodListItemResponse = {
  description: string;
  id: string;
  name: string;
  price: number;
};
export type GetShippingMethodListResponse = {
  shippingMethods: GetShippingMethodListItemResponse[];
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export const { useGetShippingMethodListQuery } = injectedRtkApi;
