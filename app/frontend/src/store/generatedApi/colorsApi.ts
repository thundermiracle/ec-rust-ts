import { api } from "./baseApi";
export const addTagTypes = ["Colors"] as const;
const injectedRtkApi = api
  .enhanceEndpoints({
    addTagTypes,
  })
  .injectEndpoints({
    endpoints: (build) => ({
      getColorList: build.query<GetColorListApiResponse, GetColorListApiArg>({
        query: () => ({ url: `/colors` }),
        providesTags: ["Colors"],
      }),
    }),
    overrideExisting: false,
  });
export { injectedRtkApi as enhancedApi };
export type GetColorListApiResponse =
  /** status 200 色一覧の取得成功 */ ColorListResponse;
export type GetColorListApiArg = void;
export type ColorListItemResponse = {
  hex: string;
  id: number;
  name: string;
};
export type ColorListResponse = {
  colors: ColorListItemResponse[];
};
export type ErrorResponse = {
  /** エラーコード */
  code: string;
  /** エラー詳細（任意） */
  details?: string;
  /** エラーメッセージ */
  message: string;
};
export const { useGetColorListQuery } = injectedRtkApi;
