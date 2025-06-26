'use client';

import React from 'react';
import { Drawer, DrawerContent, DrawerHeader, DrawerTitle, DrawerClose } from '@/components/ui/drawer';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { Trash2, Plus, Minus } from 'lucide-react';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { selectCartItems, selectCartIsOpen, selectCartTotal, setCartOpen, updateQuantity, removeFromCart } from '@/store/cartSlice';
import Image from 'next/image';

export const CartDrawer: React.FC = () => {
  const dispatch = useAppDispatch();
  const cartItems = useAppSelector(selectCartItems);
  const isOpen = useAppSelector(selectCartIsOpen);
  const total = useAppSelector(selectCartTotal);

  const handleClose = () => {
    dispatch(setCartOpen(false));
  };

  const handleUpdateQuantity = (id: string, skuId: string | undefined, quantity: number) => {
    dispatch(updateQuantity({ id, skuId, quantity }));
  };

  const handleRemoveItem = (id: string, skuId: string | undefined) => {
    dispatch(removeFromCart({ id, skuId }));
  };

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
          ) : (
            <div className="space-y-4">
              {cartItems.map((item) => (
                <div key={`${item.id}-${item.skuId}`} className="flex gap-4 p-4 border rounded-lg">
                  <div className="relative w-16 h-16 flex-shrink-0">
                    <Image
                      src={item.image || '/images/placeholder.jpg'}
                      alt={item.name}
                      fill
                      className="object-cover rounded"
                    />
                  </div>

                  <div className="flex-1 min-w-0">
                    <h3 className="font-medium text-sm truncate">{item.name}</h3>
                    {item.color && (
                      <p className="text-xs text-muted-foreground">Color: {item.color}</p>
                    )}
                    {item.size && (
                      <p className="text-xs text-muted-foreground">Size: {item.size}</p>
                    )}
                    <div className="mt-1">
                      {item.salePrice ? (
                        <div className="flex items-center gap-2">
                          <p className="font-semibold text-sm text-red-600">¥{item.salePrice.toLocaleString()}</p>
                          <p className="text-xs text-muted-foreground line-through">¥{item.price.toLocaleString()}</p>
                        </div>
                      ) : (
                        <p className="font-semibold text-sm">¥{item.price.toLocaleString()}</p>
                      )}
                    </div>
                  </div>

                  <div className="flex flex-col items-end gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleRemoveItem(item.id, item.skuId)}
                      className="h-6 w-6 p-0 text-muted-foreground hover:text-destructive"
                    >
                      <Trash2 className="h-3 w-3" />
                    </Button>

                    <div className="flex items-center gap-1">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleUpdateQuantity(item.id, item.skuId, item.quantity - 1)}
                        className="h-6 w-6 p-0"
                        disabled={item.quantity <= 1}
                      >
                        <Minus className="h-3 w-3" />
                      </Button>
                      <span className="text-xs font-medium w-8 text-center">{item.quantity}</span>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleUpdateQuantity(item.id, item.skuId, item.quantity + 1)}
                        className="h-6 w-6 p-0"
                      >
                        <Plus className="h-3 w-3" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {cartItems.length > 0 && (
          <>
            <Separator />
            <div className="p-4 space-y-4">
              <div className="flex items-center justify-between">
                <span className="font-semibold">Total:</span>
                <span className="font-bold text-lg">¥{total.toLocaleString()}</span>
              </div>
              
              <div className="space-y-2">
                <Button className="w-full cursor-pointer" size="lg">
                  Checkout
                </Button>
                <Button variant="outline" className="w-full cursor-pointer" onClick={handleClose}>
                  Continue Shopping
                </Button>
              </div>
            </div>
          </>
        )}
      </DrawerContent>
    </Drawer>
  );
}; 