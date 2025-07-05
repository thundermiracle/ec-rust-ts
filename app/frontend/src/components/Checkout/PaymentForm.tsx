import React from 'react';
import { useFormContext, Controller } from 'react-hook-form';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { CreditCard } from 'lucide-react';
import { PAYMENT_OPTIONS } from './mockData';
import { type PaymentFormData } from '@/lib/validators/checkout';
import { FormInputField } from '@/components/ui/form-input-field';
import { FormRadioGroup } from '@/components/ui/form-radio-group';

interface PaymentFormProps {
  onPrev: () => void;
  onNext: () => void;
}

export function PaymentForm({ onPrev, onNext }: PaymentFormProps) {
  const { control, watch } = useFormContext<PaymentFormData>();
  const paymentMethod = watch('paymentMethod');

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center">
          <CreditCard className="h-5 w-5 mr-2" />
          支払い情報
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <FormRadioGroup
          name="paymentMethod"
          label="支払い方法"
          options={PAYMENT_OPTIONS.map(option => ({
            id: option.id,
            label: option.name,
            description: option.description
          }))}
        />

        {/* クレジットカード情報 */}
        {paymentMethod === 'credit' && (
          <div className="space-y-4">
            <FormInputField
              name="cardNumber"
              label="カード番号*"
              placeholder="1234 5678 9012 3456"
            />
            <div className="grid grid-cols-2 gap-4">
              <FormInputField name="expiryDate" label="有効期限*" placeholder="MM/YY" />
              <FormInputField name="cvc" label="CVC*" placeholder="123" />
            </div>
            <FormInputField
              name="cardName"
              label="カード名義*"
              placeholder="YAMADA TARO"
            />
          </div>
        )}

        <Controller
          name="saveInfo"
          control={control}
          render={({ field }) => (
            <Checkbox
              id="saveInfo"
              label="次回のために情報を保存する"
              checked={field.value || false}
              onChange={field.onChange}
            />
          )}
        />
        
        <div className="flex space-x-4">
          <Button 
            variant="outline" 
            className="flex-1"
            onClick={onPrev}
          >
            戻る
          </Button>
          <Button 
            className="flex-1"
            onClick={onNext}
          >
            注文内容を確認
          </Button>
        </div>
      </CardContent>
    </Card>
  );
} 