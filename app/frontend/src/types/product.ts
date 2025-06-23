export interface ProductListItem {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  image: string;
  category: string;
  colors: string[];
  isOnSale?: boolean;
  isBestSeller?: boolean;
  isQuickShip?: boolean;
  isSoldOut?: boolean;
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
  parentId?: string;
  displayOrder: number;
  subcategories?: Category[];
}

export interface CategoryList {
  categories: Category[];
}

export interface Product {
  id: string;
  name: string;
  images: string[];
  category: string;
  description: string;
  isBestSeller: boolean;
  isQuickShip: boolean;
  variants: ProductVariant[];
}

export interface ProductVariant {
  id: string;
  skuCode: string;
  name: string;
  color: string;
  material: string;
  dimensions: string;
  price: number;
  salePrice?: number;
  displayOrder: number;
  image?: string;
  isOnSale: boolean;
  isSoldOut: boolean;
}