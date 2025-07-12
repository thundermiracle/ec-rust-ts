import { api } from "./baseApi";
export const addTagTypes = ["PaymentMethods"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      getPaymentMethodList: build.query<
        GetPaymentMethodListApiResponse,
        GetPaymentMethodListApiArg
      >({
        query: () => ({ url: `/payment-methods` }),
        providesTags: ["PaymentMethods"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type GetPaymentMethodListApiResponse =
  /** status 200 PaymentMethodリスト取得成功 */ GetPaymentMethodListResponse;
export type GetPaymentMethodListApiArg = void;
export type PaymentMethodListItemResponse = {
  description?: string;
  id: string;
  name?: string;
};
export type GetPaymentMethodListResponse = {
  items: PaymentMethodListItemResponse[];
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export const { useGetPaymentMethodListQuery } = injectedRtkApi;
