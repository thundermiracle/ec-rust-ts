'use client';

import React, { useMemo } from 'react';
import { useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { Drawer, DrawerContent, DrawerHeader, DrawerTitle, DrawerClose } from '@/components/ui/drawer';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Separator } from '@/components/ui/separator';
import { Trash2, Plus, Minus, Ticket } from 'lucide-react';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { 
  selectCartItems, 
  selectCartIsOpen, 
  setCartOpen, 
  updateQuantity, 
  removeFromCart
} from '@/store/cartSlice';
import { useGetProductListQuery, useFindVariantsMutation, useCalculateCartMutation } from '@/store/api';
import Image from 'next/image';
import { enhanceCartItemsWithVariantAPI, calculateCartTotal } from './helper';

export const CartDrawer: React.FC = () => {
  const router = useRouter();
  const dispatch = useAppDispatch();
  const cartItems = useAppSelector(selectCartItems);
  const isOpen = useAppSelector(selectCartIsOpen);

  // クーポンフォーム管理
  interface CouponForm {
    couponCode: string;
  }
  
  const { register, handleSubmit, reset, formState: { isValid } } = useForm<CouponForm>({
    defaultValues: { couponCode: '' },
    mode: 'onChange'
  });
  
  // couponCode は現在使用していないが、必要に応じて可能
  // const couponCode = watch('couponCode');
  
  // カート計算のmutation（手動トリガー用）
  const [triggerCalculation, cartCalculationResult] = useCalculateCartMutation();
  
  // カート計算データの取得
  const { data: cartCalculationData, isLoading: isCartCalculationLoading, error: cartCalculationError } = cartCalculationResult;

  // 商品リストを取得（カートにある商品の基本情報を取得）
  const { data: productListData, isLoading: isProductListLoading } = useGetProductListQuery();

  // バリアント詳細を取得するためのmutation
  const [findVariants, { data: variantsData, isLoading: isVariantsLoading }] = useFindVariantsMutation();

  // SKU IDsをソートした文字列として管理（変更検知用）
  const skuIdsKey = useMemo(() => {
    const uniqueIds = [...new Set(cartItems.map(item => item.skuId))].sort();
    return uniqueIds.join(',');
  }, [cartItems]);

  // SKU IDsのキーが変更された時のみバリアント詳細を取得
  React.useEffect(() => {
    if (isOpen && skuIdsKey) {
      const skuIds = skuIdsKey.split(',').filter(Boolean);
      if (skuIds.length > 0) {
        findVariants({ findVariantsRequest: { skuIds } });
      }
    }
  }, [isOpen, skuIdsKey, findVariants]);

  // 商品データとバリアントデータを結合
  const enhancedCartItems = useMemo(() => {
    if (!productListData?.products || !variantsData?.variants) return [];

    return enhanceCartItemsWithVariantAPI(
      cartItems, 
      productListData.products, 
      variantsData.variants
    );
  }, [cartItems, productListData, variantsData]);

  // 合計金額を計算（クーポン適用時は計算結果を使用）
  const total = useMemo(() => {
    if (cartCalculationData) {
      return cartCalculationData.total;
    }
    return calculateCartTotal(enhancedCartItems);
  }, [enhancedCartItems, cartCalculationData]);
  
  // クーポン適用処理
  const handleCouponApply = handleSubmit((data) => {
    if (data.couponCode.trim() && cartItems.length > 0) {
      triggerCalculation({
        calculateCartRequest: {
          items: cartItems.map(item => ({
            skuId: item.skuId,
            quantity: item.quantity,
          })),
          shipping_method_id: 'standard',
          payment_method_id: 'credit_card',
          coupon_code: data.couponCode.trim(),
        }
      });
    }
  });
  
  // クーポン削除処理
  const handleCouponRemove = () => {
    reset();
    // クーポンなしで再計算
    triggerCalculation({
      calculateCartRequest: {
        items: cartItems.map(item => ({
          skuId: item.skuId,
          quantity: item.quantity,
        })),
        shipping_method_id: 'standard',
        payment_method_id: 'credit_card',
        coupon_code: null,
      }
    });
  };

  const handleClose = () => {
    dispatch(setCartOpen(false));
  };

  const handleUpdateQuantity = (productId: string, skuId: string, quantity: number) => {
    dispatch(updateQuantity({ productId, skuId, quantity }));
  };

  const handleRemoveItem = (productId: string, skuId: string) => {
    dispatch(removeFromCart({ productId, skuId }));
  };

  const isLoading = isProductListLoading || isVariantsLoading;

  return (
    <Drawer open={isOpen} onOpenChange={handleClose} side="right">
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Shopping Cart ({cartItems.length})</DrawerTitle>
          <DrawerClose onClick={handleClose} />
        </DrawerHeader>

        <div className="flex-1 overflow-y-auto p-4">
          {cartItems.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center">
              <p className="text-muted-foreground mb-4">Your cart is empty</p>
              <Button onClick={handleClose} variant="outline">
                Continue Shopping
              </Button>
            </div>
          ) : isLoading ? (
            <div className="flex flex-col items-center justify-center h-64 text-center">
              <p className="text-muted-foreground mb-4">Loading cart items...</p>
            </div>
          ) : (
            <div className="space-y-4">
              {enhancedCartItems.map((item) => (
                <div key={`${item.productId}-${item.skuId}`} className="flex gap-4 p-4 border rounded-lg">
                  <div className="relative w-16 h-16 flex-shrink-0">
                    <Image
                      src={item.image || '/images/placeholder.jpg'}
                      alt={item.name || 'Product'}
                      fill
                      className="object-cover rounded"
                    />
                  </div>

                  <div className="flex-1 min-w-0">
                    <h3 className="font-medium text-sm truncate">
                      {item.isAvailable ? item.name : `${item.name} (Unavailable)`}
                    </h3>                    
                    {/* バリアント詳細情報を表示 */}
                    {item.material && (
                      <p className="text-xs text-muted-foreground">Material: {item.material}</p>
                    )}
                    {item.dimensions && (
                      <p className="text-xs text-muted-foreground">Dimensions: {item.dimensions}</p>
                    )}
                    
                    {!item.isAvailable && (
                      <p className="text-xs text-red-500 mt-1">This item is no longer available</p>
                    )}
                    
                    {item.isAvailable && (
                      <div className="mt-1">
                        {item.salePrice && item.price ? (
                          <div className="flex items-center gap-2">
                            <p className="font-semibold text-sm text-red-600">¥{item.salePrice.toLocaleString()}</p>
                            <p className="text-xs text-muted-foreground line-through">¥{item.price.toLocaleString()}</p>
                          </div>
                        ) : (
                          <p className="font-semibold text-sm">¥{(item.price || 0).toLocaleString()}</p>
                        )}
                      </div>
                    )}
                  </div>

                  <div className="flex flex-col items-end gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleRemoveItem(item.productId, item.skuId)}
                      className="h-6 w-6 p-0 text-muted-foreground hover:text-destructive"
                    >
                      <Trash2 className="h-3 w-3" />
                    </Button>

                    {item.isAvailable && (
                      <div className="flex items-center gap-1">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleUpdateQuantity(item.productId, item.skuId, item.quantity - 1)}
                          className="h-6 w-6 p-0"
                          disabled={item.quantity <= 1}
                        >
                          <Minus className="h-3 w-3" />
                        </Button>
                        <span className="text-xs font-medium w-8 text-center">{item.quantity}</span>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleUpdateQuantity(item.productId, item.skuId, item.quantity + 1)}
                          className="h-6 w-6 p-0"
                        >
                          <Plus className="h-3 w-3" />
                        </Button>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {cartItems.length > 0 && !isLoading && (
          <>
            <Separator />
            <div className="p-4 space-y-4">
              {/* クーポン入力セクション */}
              <form onSubmit={handleCouponApply} className="space-y-2">
                <div className="flex gap-2">
                  <div className="flex-1">
                    <Input
                      {...register('couponCode', { 
                        required: true,
                        setValueAs: (value: string) => value.toUpperCase()
                      })}
                      placeholder="クーポンコードを入力"
                      className="text-sm"
                      disabled={!!cartCalculationData?.appliedCoupon}
                    />
                  </div>
                  <Button
                    type="submit"
                    variant="outline"
                    size="sm"
                    disabled={!isValid || !!cartCalculationData?.appliedCoupon || isCartCalculationLoading}
                    className="flex items-center gap-1"
                  >
                    <Ticket className="h-3 w-3" />
                    {isCartCalculationLoading ? '適用中...' : '適用'}
                  </Button>
                </div>
                
                {cartCalculationData?.appliedCoupon && (
                  <div className="flex items-center justify-between text-sm text-green-600 bg-green-50 px-2 py-1 rounded">
                    <span>クーポン適用: {cartCalculationData.appliedCoupon.couponName} (-¥{cartCalculationData.appliedCoupon.discountAmount.toLocaleString()})</span>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={handleCouponRemove}
                      className="h-6 w-6 p-0 hover:bg-green-100"
                    >
                      ×
                    </Button>
                  </div>
                )}
                
                {cartCalculationData?.couponError && (
                  <div className="text-sm text-red-600 bg-red-50 px-2 py-1 rounded">
                    {cartCalculationData.couponError.errorMessage}
                  </div>
                )}
                
                {cartCalculationError && (
                  <div className="text-sm text-red-600 bg-red-50 px-2 py-1 rounded">
                    クーポンの処理中にエラーが発生しました
                  </div>
                )}
              </form>
              
              {/* 割引情報を表示 */}
              {cartCalculationData?.appliedCoupon && (
                <div className="space-y-1">
                  <div className="flex items-center justify-between text-sm">
                    <span>小計:</span>
                    <span>¥{cartCalculationData.subtotal.toLocaleString()}</span>
                  </div>
                  <div className="flex items-center justify-between text-sm text-green-600">
                    <span>クーポン割引 ({cartCalculationData.appliedCoupon.couponName}):</span>
                    <span>-¥{cartCalculationData.appliedCoupon.discountAmount.toLocaleString()}</span>
                  </div>
                  {cartCalculationData.shippingFee > 0 && (
                    <div className="flex items-center justify-between text-sm">
                      <span>配送料:</span>
                      <span>¥{cartCalculationData.shippingFee.toLocaleString()}</span>
                    </div>
                  )}
                  {cartCalculationData.paymentFee > 0 && (
                    <div className="flex items-center justify-between text-sm">
                      <span>手数料:</span>
                      <span>¥{cartCalculationData.paymentFee.toLocaleString()}</span>
                    </div>
                  )}
                  <div className="flex items-center justify-between text-sm">
                    <span>税額:</span>
                    <span>¥{cartCalculationData.taxAmount.toLocaleString()}</span>
                  </div>
                </div>
              )}
              
              <div className="flex items-center justify-between">
                <span className="font-semibold">Total:</span>
                <span className="font-bold text-lg">¥{total.toLocaleString()}</span>
              </div>
              
              <div className="space-y-2">
                <Button 
                  className="w-full cursor-pointer" 
                  size="lg"
                  disabled={enhancedCartItems.some(item => !item.isAvailable)}
                  onClick={() => {
                    if (!enhancedCartItems.some(item => !item.isAvailable)) {
                      handleClose();
                      router.push('/checkout');
                    }
                  }}
                >
                  Checkout
                </Button>
                <Button variant="outline" className="w-full cursor-pointer" onClick={handleClose}>
                  Continue Shopping
                </Button>
              </div>
              
              {enhancedCartItems.some(item => !item.isAvailable) && (
                <p className="text-xs text-muted-foreground text-center">
                  Please remove unavailable items before checkout
                </p>
              )}
            </div>
          </>
        )}
      </DrawerContent>
    </Drawer>
  );
}; 