import { enhanceCartItems, enhanceCartItemsWithVariants, calculateCartTotal, type CartItem } from './helper';
import { type StoredCartItem } from '@/store/cartSlice';
import { type ProductListItemResponse, type VariantResponse } from '@/store/generatedApi/productsApi';

describe('Cart Helper Functions', () => {
  describe('enhanceCartItems', () => {
    const mockCartItems: StoredCartItem[] = [
      { productId: '1', skuId: 'sku-1', quantity: 2 },
      { productId: '2', skuId: 'sku-2', quantity: 1 },
      { productId: '3', skuId: 'sku-3', quantity: 3 }, // この商品は存在しない
    ];

    const mockProducts: ProductListItemResponse[] = [
      {
        id: '1',
        name: 'Product 1',
        price: 1000,
        salePrice: 800,
        image: '/images/product1.jpg',
        category: 'Category 1',
        colors: ['red'],
        isSoldOut: false,
      },
      {
        id: '2',
        name: 'Product 2',
        price: 2000,
        image: '/images/product2.jpg',
        category: 'Category 2',
        colors: ['blue'],
        isSoldOut: true, // 在庫切れ
      },
    ];

    it('should enhance cart items with product data correctly', () => {
      const result = enhanceCartItems(mockCartItems, mockProducts);

      expect(result).toHaveLength(3);
      
      // Product 1 - セール価格あり、在庫あり
      expect(result[0]).toEqual({
        productId: '1',
        skuId: 'sku-1',
        quantity: 2,
        name: 'Product 1',
        price: 1000,
        salePrice: 800,
        image: '/images/product1.jpg',
        isAvailable: true,
      });

      // Product 2 - 在庫切れ
      expect(result[1]).toEqual({
        productId: '2',
        skuId: 'sku-2',
        quantity: 1,
        name: 'Product 2',
        price: 2000,
        salePrice: undefined,
        image: '/images/product2.jpg',
        isAvailable: false,
      });

      // Product 3 - 存在しない商品
      expect(result[2]).toEqual({
        productId: '3',
        skuId: 'sku-3',
        quantity: 3,
        isAvailable: false,
        name: 'Product not found',
        price: 0,
      });
    });

    it('should handle empty cart items', () => {
      const result = enhanceCartItems([], mockProducts);
      expect(result).toEqual([]);
    });

    it('should handle empty products list', () => {
      const result = enhanceCartItems(mockCartItems, []);
      
      expect(result).toHaveLength(3);
      result.forEach((item) => {
        expect(item.isAvailable).toBe(false);
        expect(item.name).toBe('Product not found');
        expect(item.price).toBe(0);
      });
    });
  });

  describe('enhanceCartItemsWithVariants', () => {
    const mockCartItems: StoredCartItem[] = [
      { productId: '1', skuId: 'variant-1', quantity: 2 },
      { productId: '2', skuId: 'variant-3', quantity: 1 }, // 存在しないバリエーション
      { productId: '3', skuId: 'variant-5', quantity: 1 }, // 存在しない商品
    ];

    const mockProductDetails = [
      {
        id: '1',
        name: 'Product 1',
        images: ['/images/product1-1.jpg', '/images/product1-2.jpg'],
        variants: [
          {
            id: 'variant-1',
            name: 'Variant 1',
            price: 1500,
            salePrice: 1200,
            image: '/images/variant1.jpg',
            color: 'Red',
            material: 'Wood',
            dimensions: '10x10x10',
            isSoldOut: false,
            isOnSale: true,
            displayOrder: 1,
            skuCode: 'SKU-001',
          },
          {
            id: 'variant-2',
            name: 'Variant 2',
            price: 1800,
            image: '/images/variant2.jpg',
            color: 'Blue',
            material: 'Metal',
            dimensions: '12x12x12',
            isSoldOut: true,
            isOnSale: false,
            displayOrder: 2,
            skuCode: 'SKU-002',
          },
        ] as VariantResponse[],
      },
      {
        id: '2',
        name: 'Product 2',
        images: ['/images/product2.jpg'],
        variants: [
          {
            id: 'variant-4',
            name: 'Variant 4',
            price: 2000,
            color: 'Green',
            material: 'Plastic',
            dimensions: '8x8x8',
            isSoldOut: false,
            isOnSale: false,
            displayOrder: 1,
            skuCode: 'SKU-003',
          },
        ] as VariantResponse[],
      },
    ];

    it('should enhance cart items with variant data correctly', () => {
      const result = enhanceCartItemsWithVariants(mockCartItems, mockProductDetails);

      expect(result).toHaveLength(3);

      // Product 1, Variant 1 - 正常ケース
      expect(result[0]).toEqual({
        productId: '1',
        skuId: 'variant-1',
        quantity: 2,
        name: 'Product 1',
        price: 1500,
        salePrice: 1200,
        image: '/images/variant1.jpg',
        color: 'Red',
        material: 'Wood',
        dimensions: '10x10x10',
        isAvailable: true,
      });

      // Product 2, Variant 3 - 存在しないバリエーション
      expect(result[1]).toEqual({
        productId: '2',
        skuId: 'variant-3',
        quantity: 1,
        name: 'Product 2',
        image: '/images/product2.jpg',
        isAvailable: false,
        price: 0,
      });

      // Product 3 - 存在しない商品
      expect(result[2]).toEqual({
        productId: '3',
        skuId: 'variant-5',
        quantity: 1,
        isAvailable: false,
        name: 'Product not found',
        price: 0,
      });
    });

    it('should use product image when variant image is not available', () => {
      const cartItems: StoredCartItem[] = [
        { productId: '2', skuId: 'variant-4', quantity: 1 },
      ];

      const result = enhanceCartItemsWithVariants(cartItems, mockProductDetails);

      expect(result[0].image).toBe('/images/product2.jpg'); // バリエーション画像がないので商品画像を使用
    });

    it('should handle empty cart items', () => {
      const result = enhanceCartItemsWithVariants([], mockProductDetails);
      expect(result).toEqual([]);
    });

    it('should handle empty product details', () => {
      const result = enhanceCartItemsWithVariants(mockCartItems, []);
      
      expect(result).toHaveLength(3);
      result.forEach((item) => {
        expect(item.isAvailable).toBe(false);
        expect(item.name).toBe('Product not found');
        expect(item.price).toBe(0);
      });
    });
  });

  describe('calculateCartTotal', () => {
    it('should calculate total correctly with regular prices', () => {
      const cartItems: CartItem[] = [
        {
          productId: '1',
          skuId: 'sku-1',
          quantity: 2,
          price: 1000,
          isAvailable: true,
        },
        {
          productId: '2',
          skuId: 'sku-2',
          quantity: 3,
          price: 500,
          isAvailable: true,
        },
      ];

      const total = calculateCartTotal(cartItems);
      expect(total).toBe(3500); // (1000 * 2) + (500 * 3)
    });

    it('should prioritize sale price over regular price', () => {
      const cartItems: CartItem[] = [
        {
          productId: '1',
          skuId: 'sku-1',
          quantity: 2,
          price: 1000,
          salePrice: 800,
          isAvailable: true,
        },
        {
          productId: '2',
          skuId: 'sku-2',
          quantity: 1,
          price: 500,
          isAvailable: true,
        },
      ];

      const total = calculateCartTotal(cartItems);
      expect(total).toBe(2100); // (800 * 2) + (500 * 1)
    });

    it('should handle items with no price', () => {
      const cartItems: CartItem[] = [
        {
          productId: '1',
          skuId: 'sku-1',
          quantity: 2,
          isAvailable: false, // 価格情報なし（削除された商品など）
        },
        {
          productId: '2',
          skuId: 'sku-2',
          quantity: 1,
          price: 1000,
          isAvailable: true,
        },
      ];

      const total = calculateCartTotal(cartItems);
      expect(total).toBe(1000); // 価格なしのアイテムは0として計算
    });

    it('should handle empty cart', () => {
      const total = calculateCartTotal([]);
      expect(total).toBe(0);
    });

    it('should handle mixed available and unavailable items', () => {
      const cartItems: CartItem[] = [
        {
          productId: '1',
          skuId: 'sku-1',
          quantity: 2,
          price: 1000,
          isAvailable: true,
        },
        {
          productId: '2',
          skuId: 'sku-2',
          quantity: 1,
          price: 500,
          isAvailable: false, // 在庫切れでも価格は計算に含める
        },
      ];

      const total = calculateCartTotal(cartItems);
      expect(total).toBe(2500); // 在庫状況に関係なく価格は計算される
    });

    it('should handle decimal prices correctly', () => {
      const cartItems: CartItem[] = [
        {
          productId: '1',
          skuId: 'sku-1',
          quantity: 3,
          price: 333.33,
          isAvailable: true,
        },
      ];

      const total = calculateCartTotal(cartItems);
      expect(total).toBeCloseTo(999.99, 2);
    });
  });
}); 