'use client';

import React from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Ticket } from 'lucide-react';

interface CouponFormProps {
  onApply: (couponCode: string) => void;
  onRemove: () => void;
  isLoading?: boolean;
  isDisabled?: boolean;
  appliedCoupon?: {
    couponName: string;
    discountAmount: number;
  } | null;
  couponError?: {
    errorMessage: string;
  } | null;
  className?: string;
  couponCode?: string;
  setCouponCode?: (code: string) => void;
}

export function CouponForm({
  onApply,
  onRemove,
  isLoading = false,
  isDisabled = false,
  appliedCoupon,
  couponError,
  className = '',
  couponCode = '',
  setCouponCode,
}: CouponFormProps) {
  const handleCouponApply = (e?: React.FormEvent | React.KeyboardEvent) => {
    e?.preventDefault();
    e?.stopPropagation();
    if (couponCode.trim()) {
      onApply(couponCode.trim());
    }
  };

  const handleCouponRemove = () => {
    if (setCouponCode) {
      setCouponCode('');
    }
    onRemove();
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value.toUpperCase();
    if (setCouponCode) {
      setCouponCode(value);
    }
  };

  // クーポンコードが入力されているかチェック
  const isValid = couponCode.trim().length > 0;

  return (
    <div className={`space-y-2 ${className}`}>
      <div className="space-y-2">
        <div className="flex gap-2">
          <div className="flex-1">
            <Input
              value={couponCode}
              onChange={handleInputChange}
              placeholder="クーポンコードを入力"
              className="text-sm"
              disabled={!!appliedCoupon || isDisabled}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  e.stopPropagation();
                  handleCouponApply(e);
                }
              }}
            />
          </div>
          <Button
            type="button"
            variant="outline"
            size="sm"
            disabled={!isValid || !!appliedCoupon || isLoading || isDisabled}
            className="flex items-center gap-1"
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              handleCouponApply(e);
            }}
          >
            <Ticket className="h-3 w-3" />
            {isLoading ? '適用中...' : '適用'}
          </Button>
        </div>
        
        {appliedCoupon && (
          <div className="flex items-center justify-between text-sm text-green-600 bg-green-50 px-2 py-1 rounded">
            <span>クーポン適用: {appliedCoupon.couponName} (-¥{appliedCoupon.discountAmount.toLocaleString()})</span>
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
        
        {couponError && (
          <div className="text-sm text-red-600 bg-red-50 px-2 py-1 rounded">
            {couponError.errorMessage}
          </div>
        )}
      </div>
    </div>
  );
}