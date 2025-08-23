import { useEffect, useState, useCallback } from 'react';
import { useCalculateCartMutation } from '@/store/api';
import { useAppSelector } from '@/store/hooks';
import { selectCartItems } from '@/store/cartSlice';

interface UseCartCalculationOptions {
  shippingMethod?: string;
  paymentMethod?: string;
  enabled?: boolean;
}

export function useCartCalculation(options: UseCartCalculationOptions = {}) {
  const { shippingMethod, paymentMethod, enabled = true } = options;
  const cartItems = useAppSelector(selectCartItems);
  const [couponCode, setCouponCode] = useState<string | null>(null);
  
  const [calculateCart, { 
    data: cartCalculationData, 
    isLoading: isCartCalculationLoading, 
    error: cartCalculationError 
  }] = useCalculateCartMutation();

  // カート計算実行
  const triggerCalculation = useCallback((appliedCouponCode?: string | null) => {
    if (
      enabled &&
      cartItems.length > 0 && 
      shippingMethod && 
      paymentMethod
    ) {
      const calculateCartRequest = {
        items: cartItems.map(item => ({
          skuId: item.skuId,
          quantity: item.quantity,
        })),
        shipping_method_id: shippingMethod,
        payment_method_id: paymentMethod,
        ...(appliedCouponCode && { coupon_code: appliedCouponCode }),
      };
      calculateCart({ calculateCartRequest });
    }
  }, [cartItems, shippingMethod, paymentMethod, calculateCart, enabled]);

  // 自動計算実行
  useEffect(() => {
    triggerCalculation(couponCode);
  }, [triggerCalculation, couponCode]);

  // クーポン適用
  const applyCoupon = useCallback((code: string) => {
    setCouponCode(code);
    triggerCalculation(code);
  }, [triggerCalculation]);

  // クーポン削除
  const removeCoupon = useCallback(() => {
    setCouponCode(null);
    triggerCalculation(null);
  }, [triggerCalculation]);

  return {
    cartCalculationData,
    isCartCalculationLoading,
    cartCalculationError,
    total: cartCalculationData?.total || 0,
    subtotal: cartCalculationData?.subtotal || 0,
    tax: cartCalculationData?.taxAmount || 0,
    shippingFee: cartCalculationData?.shippingFee || 0,
    paymentFee: cartCalculationData?.paymentFee || 0,
    appliedCoupon: cartCalculationData?.appliedCoupon || null,
    couponError: cartCalculationData?.couponError || null,
    currentCouponCode: couponCode,
    applyCoupon,
    removeCoupon,
  };
}