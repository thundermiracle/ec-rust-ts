'use client';

import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { FC } from 'react';
import { useAppDispatch } from '@/store/hooks';
import { addToCart } from '@/store/cartSlice';

interface ProductPurchaseProps {
  quantity: number;
  onQuantityChange: (quantity: number) => void;
  isSoldOut: boolean;
  // カート追加に必要な商品情報
  product: {
    id: string;
    name: string;
  };
  selectedVariant: {
    id: string;
    price: number;
    image?: string;
    color?: string;
    size?: string;
  };
}

export const ProductPurchase: FC<ProductPurchaseProps> = ({
  quantity,
  onQuantityChange,
  isSoldOut,
  product,
  selectedVariant,
}) => {
  const dispatch = useAppDispatch();

  const handleAddToCart = () => {
    if (isSoldOut) return;

    const cartItem = {
      id: product.id,
      name: product.name,
      price: selectedVariant.price,
      image: selectedVariant.image || '',
      skuId: selectedVariant.id,
      color: selectedVariant.color,
      size: selectedVariant.size,
      quantity,
    };

    dispatch(addToCart(cartItem));
  };

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