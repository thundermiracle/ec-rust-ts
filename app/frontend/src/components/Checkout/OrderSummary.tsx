import React, { useEffect, useMemo } from 'react';
import Image from 'next/image';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { useAppSelector } from '@/store/hooks';
import { selectCartItems } from '@/store/cartSlice';
import { 
  useGetProductListQuery, 
  useFindVariantsMutation, 
  useCalculateCartMutation, 
  useGetShippingMethodListQuery 
} from '@/store/api';
import { enhanceCartItemsWithVariantAPI } from '@/components/Cart/CartDrawer/helper';
import { useFormContext } from 'react-hook-form';
import type { CheckoutFormData } from '@/lib/validators/checkout';

// 表示用アイテム型（内部用）
interface OrderSummaryItem {
  productId: string;
  skuId: string;
  name?: string;
  price?: number;
  salePrice?: number;
  quantity: number;
  image?: string;
  material?: string;
}

export function OrderSummary() {
  // カート情報
  const cartItems = useAppSelector(selectCartItems);

  // react-hook-form から配送方法を取得
  const { watch } = useFormContext<CheckoutFormData>();
  const shippingMethod = watch('shippingMethod');

  // 配送方法リスト取得
  const { data: shippingData } = useGetShippingMethodListQuery();

  // 商品リスト取得
  const { data: productListData, isLoading: isProductListLoading } = useGetProductListQuery();

  // バリアント詳細取得
  const [findVariants, { data: variantsData, isLoading: isVariantsLoading }] = useFindVariantsMutation();

  // カート計算
  const [calculateCart, { data: cartCalculationData, isLoading: isCartCalculationLoading }] = useCalculateCartMutation();

  // SKU ID 一意キー
  const skuIdsKey = useMemo(() => {
    const uniqueIds = [...new Set(cartItems.map(item => item.skuId))].sort();
    return uniqueIds.join(',');
  }, [cartItems]);

  // バリアントAPI呼び出し
  useEffect(() => {
    if (skuIdsKey) {
      const skuIds = skuIdsKey.split(',').filter(Boolean);
      if (skuIds.length > 0) {
        findVariants({ findVariantsRequest: { skuIds } });
      }
    }
  }, [skuIdsKey, findVariants]);

  // カート計算API呼び出し
  useEffect(() => {
    if (cartItems.length > 0) {
      const calculateCartRequest = {
        items: cartItems.map(item => ({
          skuId: item.skuId,
          quantity: item.quantity,
        })),
      };
      calculateCart({ calculateCartRequest });
    }
  }, [cartItems, calculateCart]);

  // UI表示用カートアイテム
  const enhancedCartItems: OrderSummaryItem[] = useMemo(() => {
    if (!productListData?.products || !variantsData?.variants) return [];
    return enhanceCartItemsWithVariantAPI(
      cartItems,
      productListData.products,
      variantsData.variants,
    ).map(item => ({
      ...item,
      name: item.name || '商品名不明',
    }));
  }, [cartItems, productListData, variantsData]);

  // 配送料計算
  const selectedShipping = shippingData?.shippingMethods.find(option => option.id === shippingMethod);
  const shippingCost = selectedShipping?.price || 0;

  // 金額計算
  const subtotal = cartCalculationData?.subtotal || 0;
  const tax = cartCalculationData?.taxAmount || 0;
  const total = (cartCalculationData?.total || 0) + shippingCost;

  // ローディング状態
  const isLoading = isProductListLoading || isVariantsLoading || isCartCalculationLoading;

  if (isLoading) {
    return (
      <Card className="sticky top-8">
        <CardHeader>
          <CardTitle>注文概要</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="text-center py-8">
            <p className="text-muted-foreground">読み込み中...</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="sticky top-8">
      <CardHeader>
        <CardTitle>注文概要</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* 商品リスト */}
        <div className="space-y-4">
          {enhancedCartItems.map((item) => (
            <div key={`${item.productId}-${item.skuId}`} className="flex items-center space-x-3">
              <div className="relative w-16 h-16 flex-shrink-0">
                <Image
                  src={item.image || '/images/placeholder.jpg'}
                  alt={item.name || 'Product'}
                  fill
                  className="object-cover rounded"
                />
                <div className="absolute -top-2 -right-2 bg-primary text-primary-foreground text-xs rounded-full h-5 w-5 flex items-center justify-center">
                  {item.quantity}
                </div>
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-medium text-sm truncate">{item.name}</h3>
                {item.material && (
                  <p className="text-xs text-muted-foreground">Material: {item.material}</p>
                )}
                <div className="flex items-center justify-between mt-1">
                  <span className="text-sm text-muted-foreground">
                    数量: {item.quantity}
                  </span>
                  <span className="font-medium text-sm">
                    ¥{((item.salePrice || item.price || 0) * item.quantity).toLocaleString()}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>

        <Separator />

        {/* 金額詳細 */}
        <div className="space-y-2">
          <div className="flex justify-between text-sm">
            <span>小計</span>
            <span>¥{subtotal.toLocaleString()}</span>
          </div>
          <div className="flex justify-between text-sm">
            <span>配送料</span>
            <span>¥{shippingCost.toLocaleString()}</span>
          </div>
          <div className="flex justify-between text-sm">
            <span>税込</span>
            <span>¥{tax.toLocaleString()}</span>
          </div>
          <Separator />
          <div className="flex justify-between font-bold">
            <span>合計</span>
            <span>¥{total.toLocaleString()}</span>
          </div>
        </div>
      </CardContent>
    </Card>
  );
} 