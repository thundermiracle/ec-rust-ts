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
  /** status 200 色一覧の取得成功 */ GetColorListResponse;
export type GetColorListApiArg = void;
export type GetColorListItemResponse = {
  hex: string;
  id: number;
  name: string;
};
export type GetColorListResponse = {
  colors: GetColorListItemResponse[];
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
