import { api } from "./baseApi";
export const addTagTypes = ["Variants"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      findVariants: build.mutation<FindVariantsApiResponse, FindVariantsApiArg>(
        {
          query: (queryArg) => ({
            url: `/variants`,
            method: "POST",
            body: queryArg.findVariantsRequest,
          }),
          invalidatesTags: ["Variants"],
        },
      ),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type FindVariantsApiResponse =
  /** status 200 バリアント詳細の取得成功 */ FindVariantsResponse;
export type FindVariantsApiArg = {
  findVariantsRequest: FindVariantsRequest;
};
export type FindVariantsItemResponse = {
  dimensions?: string;
  image?: string;
  material?: string;
  price: number;
  salePrice?: number;
  skuId: string;
};
export type FindVariantsResponse = {
  variants: FindVariantsItemResponse[];
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export type FindVariantsRequest = {
  skuIds: string[];
};
export const { useFindVariantsMutation } = injectedRtkApi;
