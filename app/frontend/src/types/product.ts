export interface Product {
  id: string;
  name: string;
  price: number;
  salePrice?: number;
  images: string[];
  category: string;
  subcategory?: string;
  description: string;
  material?: string;
  dimensions?: string;
  colors: string[];
  isOnSale?: boolean;
  isSoldOut?: boolean;
  isBestSeller?: boolean;
  isQuickShip?: boolean;
  variants?: ProductVariant[];
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