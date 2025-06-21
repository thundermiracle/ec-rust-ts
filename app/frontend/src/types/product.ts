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

export interface FilterOptions {
  categories: Category[];
  colors: string[];
  priceRange: {
    min: number;
    max: number;
  };
} 