import type { ConfigFile } from "@rtk-query/codegen-openapi";

const config: ConfigFile = {
  schemaFile: "http://localhost:4000/api-docs/openapi.json",
  apiFile: "./src/store/generatedApi/baseApi.ts",
  outputFiles: {
    "./src/store/generatedApi/productsApi.ts": {
      filterEndpoints: [/product/i],
    },
    "./src/store/generatedApi/categoriesApi.ts": {
      filterEndpoints: [/category/i],
    },
    "./src/store/generatedApi/colorsApi.ts": {
      filterEndpoints: [/color/i],
    },
    "./src/store/generatedApi/variantsApi.ts": {
      filterEndpoints: [/variant/i],
    },
    "./src/store/generatedApi/cartApi.ts": {
      filterEndpoints: [/cart/i],
    },
    "./src/store/generatedApi/shippingApi.ts": {
      filterEndpoints: [/shipping/i],
    },
    "./src/store/generatedApi/paymentApi.ts": {
      filterEndpoints: [/payment/i],
    },
    "./src/store/generatedApi/ordersApi.ts": {
      filterEndpoints: [/order/i],
    },
  },
  hooks: true,
  tag: true,
};

export default config;
