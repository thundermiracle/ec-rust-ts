export {
  useGetProductListQuery,
  useGetProductQuery,
} from "./generatedApi/productsApi";
export { useGetCategoryListQuery } from "./generatedApi/categoriesApi";
export { useGetColorListQuery } from "./generatedApi/colorsApi";

export type {
  ProductResponse,
  ProductListResponse,
  ProductListItemResponse,
  VariantResponse,
  ErrorResponse,
} from "./generatedApi/productsApi";

export type { CategoryResponse, CategoryListResponse } from "./generatedApi/categoriesApi";
export type { ColorListItemResponse, ColorListResponse } from "./generatedApi/colorsApi";