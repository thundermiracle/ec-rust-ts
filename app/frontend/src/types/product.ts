export interface ProductListItem {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  image: string;
  category: string;
  colors: string[];
  isOnSale: boolean;
  isBestSeller: boolean;
  isQuickShip: boolean;
  isSoldOut: boolean;
  stock: number;
}

export interface ProductList {
  products: ProductListItem[];
  totalCount: number;
  page: number;
  perPage: number;
  hasNextPage: boolean;
  hasPreviousPage: boolean;
}

export interface Category {
  id: string;
  name: string;
  slug: string;
  subcategories?: Category[];
}

export interface Product {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  images: string[];
  category: string;
  description: string;
  material?: string;
  dimensions?: string;
  colors: string[];
  isOnSale?: boolean;
  isBestSeller?: boolean;
  isQuickShip?: boolean;
  isSoldOut?: boolean;
  variants: ProductVariant[];
}

export interface ProductVariant {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  color: string;
  image: string;
  isAvailable: boolean;
}