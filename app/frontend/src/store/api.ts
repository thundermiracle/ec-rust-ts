export {
  useGetProductListQuery,
  useGetProductQuery,
} from "./generatedApi/productsApi";
export { useGetCategoryListQuery } from "./generatedApi/categoriesApi";
export { useGetColorListQuery } from "./generatedApi/colorsApi";
export { useGetShippingMethodListQuery } from "./generatedApi/shippingApi";
export { useFindVariantsMutation } from "./generatedApi/variantsApi";
export { useCalculateCartMutation } from "./generatedApi/cartApi";
export { useGetPaymentMethodListQuery } from "./generatedApi/paymentApi";
export { useCreateOrderMutation } from "./generatedApi/ordersApi";

export type {
  GetProductResponse,
  GetProductListResponse,
  GetProductListItemResponse,
  VariantResponse,
  ErrorResponse,
} from "./generatedApi/productsApi";
export type { FindVariantsItemResponse } from "./generatedApi/variantsApi";
export type { CategoryResponse, GetCategoryListResponse } from "./generatedApi/categoriesApi";
export type { GetColorListItemResponse, GetColorListResponse } from "./generatedApi/colorsApi";
export type { CreateOrderRequest, CreateOrderResponse, CreateOrderApiArg } from "./generatedApi/ordersApi";