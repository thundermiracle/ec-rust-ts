import { api } from "./baseApi";
export const addTagTypes = ["Orders"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      createOrder: build.mutation<CreateOrderApiResponse, CreateOrderApiArg>({
        query: (queryArg) => ({
          url: `/orders`,
          method: "POST",
          body: queryArg.createOrderRequest,
        }),
        invalidatesTags: ["Orders"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type CreateOrderApiResponse =
  /** status 201 注文作成成功 */ CreateOrderResponse;
export type CreateOrderApiArg = {
  createOrderRequest: CreateOrderRequest;
};
export type CreateOrderResponse = {
  /** 注文ID */
  order_id: string;
  /** 注文番号 */
  order_number: string;
  /** 注文ステータス */
  status: string;
  /** 合計金額（円） */
  total_amount: number;
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export type CreateOrderRequestCustomerInfo = {
  /** メールアドレス */
  email: string;
  /** 名 */
  first_name: string;
  /** 姓 */
  last_name: string;
  /** 電話番号 */
  phone: string;
};
export type CreateOrderRequestItem = {
  /** 数量 */
  quantity: number;
  /** SKU ID */
  sku_id: string;
};
export type CreateOrderRequestShippingAddress = {
  /** 建物名・部屋番号 */
  building?: string | null;
  /** 市区町村 */
  city: string;
  /** 郵便番号 */
  postal_code: string;
  /** 都道府県 */
  prefecture: string;
  /** 住所 */
  street_address: string;
};
export type CreateOrderRequest = {
  /** 顧客情報 */
  customer_info: CreateOrderRequestCustomerInfo;
  /** 注文アイテム */
  items: CreateOrderRequestItem[];
  /** 支払い方法ID */
  payment_method_id: string;
  /** 配送先住所 */
  shipping_address: CreateOrderRequestShippingAddress;
  /** 配送方法ID */
  shipping_method_id: string;
};
export const { useCreateOrderMutation } = injectedRtkApi;
