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
export type AppliedCouponResponse = {
  couponCode: string;
  couponName: string;
  discountAmount: number;
  message: string;
};
export type CouponErrorResponse = {
  couponCode?: string | null;
  errorMessage: string;
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
  appliedCoupon?: null | AppliedCouponResponse;
  couponError?: null | CouponErrorResponse;
  isEmpty: boolean;
  itemCount: number;
  items: CalculateCartItemResponse[];
  paymentFee: number;
  shippingFee: number;
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
  coupon_code?: string | null;
  items: CalculateCartItemRequest[];
  payment_method_id: string;
  shipping_method_id: string;
};
export const { useCalculateCartMutation } = injectedRtkApi;
