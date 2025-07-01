import { GetProductListItemResponse, VariantResponse } from '@/store/generatedApi/productsApi';
import { FindVariantsItemResponse } from '@/store/generatedApi/variantsApi';
import { StoredCartItem } from '@/store/cartSlice';

// UI表示用のカートアイテム
export interface CartItem {
  productId: string;
  skuId: string;
  quantity: number;
  name?: string;
  price?: number;
  salePrice?: number;
  image?: string;
  color?: string;
  material?: string;
  dimensions?: string;
  isAvailable?: boolean;  // 商品やバリエーションが削除されていないか
}

// 商品データとカートアイテムを結合してUI表示用のデータを作成
export const enhanceCartItems = (
  cartItems: StoredCartItem[], 
  products: GetProductListItemResponse[]
): CartItem[] => {
  return cartItems.map((cartItem) => {
    const product = products.find(p => p.id === cartItem.productId);
    
    if (!product) {
      // 商品が見つからない場合（削除された商品）
      return {
        ...cartItem,
        isAvailable: false,
        name: 'Product not found',
        price: 0,
      };
    }

    // 商品リストAPIからは基本情報のみ取得（バリエーション詳細は含まれない）
    return {
      ...cartItem,
      name: product.name,
      price: product.price,
      salePrice: product.salePrice,
      image: product.image,
      isAvailable: !product.isSoldOut,
    };
  });
};

// バリエーション情報を含む商品データとカートアイテムを結合
export const enhanceCartItemsWithVariants = (
  cartItems: StoredCartItem[],
  productDetails: Array<{ id: string; name: string; variants?: VariantResponse[]; images: string[] }>
): CartItem[] => {
  return cartItems.map((cartItem) => {
    const product = productDetails.find(p => p.id === cartItem.productId);
    
    if (!product) {
      return {
        ...cartItem,
        isAvailable: false,
        name: 'Product not found',
        price: 0,
      };
    }

    // バリエーションを検索
    const variant = product.variants?.find(v => v.id === cartItem.skuId);
    if (!variant) {
      return {
        ...cartItem,
        name: product.name,
        image: product.images[0],
        isAvailable: false,
        price: 0,
      };
    }

    return {
      ...cartItem,
      name: product.name,
      price: variant.price,
      salePrice: variant.salePrice,
      image: variant.image || product.images[0],
      color: variant.color,
      material: variant.material,
      dimensions: variant.dimensions,
      isAvailable: !variant.isSoldOut,
    };
  });
};

// バリアントAPIレスポンスを使用してカートアイテムを強化
export const enhanceCartItemsWithVariantAPI = (
  cartItems: StoredCartItem[],
  products: GetProductListItemResponse[],
  variantDetails: FindVariantsItemResponse[]
): CartItem[] => {
  return cartItems.map((cartItem) => {
    const product = products.find(p => p.id === cartItem.productId);
    
    if (!product) {
      // 商品が見つからない場合（削除された商品）
      return {
        ...cartItem,
        isAvailable: false,
        name: 'Product not found',
        price: 0,
      };
    }

    // バリアント詳細を検索
    const variantDetail = variantDetails.find(v => v.skuId === cartItem.skuId);
    
    if (!variantDetail) {
      // バリアントが見つからない場合（削除されたバリアント）
      return {
        ...cartItem,
        name: product.name,
        image: product.image,
        isAvailable: false,
        price: 0,
      };
    }

    return {
      ...cartItem,
      name: product.name,
      price: variantDetail.price,
      salePrice: variantDetail.salePrice,
      image: variantDetail.image || product.image,
      material: variantDetail.material,
      dimensions: variantDetail.dimensions,
      isAvailable: true, // バリアントが見つかった場合は利用可能
    };
  });
};

// 合計金額の計算
export const calculateCartTotal = (cartItems: CartItem[]) => 
  cartItems.reduce((total, item) => {
    const price = item.salePrice ?? item.price ?? 0;
    return total + (price * item.quantity);
  }, 0); 