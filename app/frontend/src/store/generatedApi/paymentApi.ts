import { api as baseApi } from "./baseApi";
const injectedRtkApi = baseApi.injectEndpoints({
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
export { injectedRtkApi as paymentApi };
export type GetPaymentMethodListApiResponse =
  /** status 200 PaymentMethodリスト取得成功 */ GetPaymentMethodListResponse;
export type GetPaymentMethodListApiArg = void;
export type GetPaymentMethodListResponse = {
  items: PaymentMethodListItemResponse[];
};
export type PaymentMethodListItemResponse = {
  id: string;
  name?: string;
  description?: string;
};
export const {
  useGetPaymentMethodListQuery,
  useLazyGetPaymentMethodListQuery,
} = injectedRtkApi;