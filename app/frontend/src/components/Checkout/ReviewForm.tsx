import React from 'react';
import { useFormContext, Controller } from 'react-hook-form';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { Textarea } from '@/components/ui/textarea';
import { Shield } from 'lucide-react';
import { useGetShippingMethodListQuery } from '@/store/api';
import { PAYMENT_OPTIONS } from './mockData';
import { type CheckoutFormData } from '@/lib/validators/checkout';

interface ReviewFormProps {
  onBack: () => void;
  total?: number;
}

// TODO: totalを動くようにする
export function ReviewForm({ onBack, total = 0 }: ReviewFormProps) {
  const { register, control, getValues, formState: { isSubmitting, errors } } = useFormContext<CheckoutFormData>();
  const { data: shippingData } = useGetShippingMethodListQuery();
  const values = getValues();

  const selectedShipping = shippingData?.shippingMethods.find(option => option.id === values.shippingMethod);
  const selectedPayment = PAYMENT_OPTIONS.find(option => option.id === values.paymentMethod);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center">
          <Shield className="h-5 w-5 mr-2" />
          注文内容の確認
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* 配送先情報 */}
        <div>
          <h3 className="font-medium mb-2">配送先</h3>
          <div className="text-sm text-muted-foreground">
            <p>{values.lastName} {values.firstName}</p>
            <p>{values.email}</p>
            <p>〒{values.postalCode}</p>
            <p>{values.prefecture} {values.city}</p>
            <p>{values.address}</p>
            {values.apartment && <p>{values.apartment}</p>}
            <p>{values.phone}</p>
          </div>
        </div>

        {/* 配送方法 */}
        <div>
          <h3 className="font-medium mb-2">配送方法</h3>
          <p className="text-sm text-muted-foreground">
            {selectedShipping?.name} - {selectedShipping?.description}
          </p>
        </div>

        {/* 支払い方法 */}
        <div>
          <h3 className="font-medium mb-2">支払い方法</h3>
          <p className="text-sm text-muted-foreground">
            {selectedPayment?.name}
          </p>
        </div>

        <Textarea
          id="notes"
          label="注文メモ（任意）"
          placeholder="配送に関するご要望などがございましたらご記入ください"
          className="min-h-[80px]"
          {...register('notes')}
        />
        {errors.notes && <p className="text-red-500 text-sm mt-1">{errors.notes.message}</p>}
        
        <Controller
          name="subscribeNewsletter"
          control={control}
          render={({ field }) => (
            <Checkbox
              id="newsletter"
              label="ニュースレターを購読する"
              checked={field.value || false}
              onChange={field.onChange}
            />
          )}
        />
        {errors.subscribeNewsletter && <p className="text-red-500 text-sm mt-1">{errors.subscribeNewsletter.message}</p>}

        <div className="flex space-x-4">
          <Button 
            variant="outline" 
            className="flex-1"
            onClick={onBack}
            type="button"
          >
            戻る
          </Button>
          <Button 
            type="submit"
            className="flex-1"
            disabled={isSubmitting}
          >
            {isSubmitting ? '注文処理中...' : `¥${total.toLocaleString()}で注文確定`}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
} 