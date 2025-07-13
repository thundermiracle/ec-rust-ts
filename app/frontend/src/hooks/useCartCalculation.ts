import { useEffect } from 'react';
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
  
  const [calculateCart, { 
    data: cartCalculationData, 
    isLoading: isCartCalculationLoading, 
    error: cartCalculationError 
  }] = useCalculateCartMutation();

  // カート計算実行
  useEffect(() => {
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
      };
      calculateCart({ calculateCartRequest });
    }
  }, [cartItems, shippingMethod, paymentMethod, calculateCart, enabled]);

  return {
    cartCalculationData,
    isCartCalculationLoading,
    cartCalculationError,
    total: cartCalculationData?.total || 0,
    subtotal: cartCalculationData?.subtotal || 0,
    tax: cartCalculationData?.taxAmount || 0,
    shippingFee: cartCalculationData?.shippingFee || 0,
    paymentFee: cartCalculationData?.paymentFee || 0,
  };
}