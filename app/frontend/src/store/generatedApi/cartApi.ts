import { api } from "./baseApi";
export const addTagTypes = ["Cart"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      calculateCart: build.mutation<
        CalculateCartApiResponse,
        CalculateCartApiArg
      >({
        query: (queryArg) => ({
          url: `/cart`,
          method: "POST",
          body: queryArg.calculateCartRequest,
        }),
        invalidatesTags: ["Cart"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type CalculateCartApiResponse =
  /** status 200 カート計算成功 */ CalculateCartResponse;
export type CalculateCartApiArg = {
  calculateCartRequest: CalculateCartRequest;
};
export type CalculateCartItemResponse = {
  productId: string;
  productName: string;
  quantity: number;
  skuId: string;
  subtotal: number;
  unitPrice: number;
};
export type CalculateCartResponse = {
  isEmpty: boolean;
  itemCount: number;
  items: CalculateCartItemResponse[];
  subtotal: number;
  taxAmount: number;
  total: number;
  totalQuantity: number;
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export type CalculateCartItemRequest = {
  quantity: number;
  skuId: string;
};
export type CalculateCartRequest = {
  items: CalculateCartItemRequest[];
};
export const { useCalculateCartMutation } = injectedRtkApi;
