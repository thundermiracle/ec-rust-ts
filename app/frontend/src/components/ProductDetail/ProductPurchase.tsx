'use client';

import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { FC, useCallback } from 'react';
import { useAppDispatch } from '@/store/hooks';
import { addToCart } from '@/store/cartSlice';

interface ProductPurchaseProps {
  quantity: number;
  onQuantityChange: (quantity: number) => void;
  isSoldOut: boolean;
  // カート追加に必要な最小限の情報
  productId: string;
  skuId: string;
}

export const ProductPurchase: FC<ProductPurchaseProps> = ({
  quantity,
  onQuantityChange,
  isSoldOut,
  productId,
  skuId,
}) => {
  const dispatch = useAppDispatch();

  const handleAddToCart = useCallback(() => {
    if (isSoldOut) return;

    // カートには productId, skuId, quantity のみ送信
    dispatch(addToCart({
      productId,
      skuId,
      quantity,
    }));
  }, [dispatch, isSoldOut, productId, skuId, quantity]);

  return (
    <div className="space-y-6">
      <div>
        <label
          htmlFor="quantity"
          className="block text-sm font-medium text-foreground mb-2"
        >
          Quantity
        </label>
        <Select
          value={quantity.toString()}
          onValueChange={(value) => onQuantityChange(Number(value))}
        >
          <SelectTrigger className="w-24">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {[...Array(10)].map((_, i) => (
              <SelectItem key={i + 1} value={(i + 1).toString()}>
                {i + 1}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <Button
        disabled={isSoldOut}
        size="lg"
        className="w-full cursor-pointer"
        onClick={handleAddToCart}
      >
        {isSoldOut ? "Out of Stock" : "Add to Cart"}
      </Button>
    </div>
  );
}; 