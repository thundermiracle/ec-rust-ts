import React from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Truck } from 'lucide-react';
import { useGetShippingMethodListQuery } from '@/store/api';
import { FormInputField } from '@/components/ui/form-input-field';
import { FormRadioGroup } from '@/components/ui/form-radio-group';

interface ShippingFormProps {
  onNext: () => void;
}

export function ShippingForm({ onNext }: ShippingFormProps) {
  const { data: shippingData, isLoading, error } = useGetShippingMethodListQuery();

  // ローディング状態やエラーハンドリング
  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center">
            <Truck className="h-5 w-5 mr-2" />
            配送情報
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="text-center py-8">
            <p className="text-muted-foreground">配送方法を読み込み中...</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center">
            <Truck className="h-5 w-5 mr-2" />
            配送情報
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="text-center py-8">
            <p className="text-red-500">配送方法の読み込みに失敗しました</p>
          </div>
        </CardContent>
      </Card>
    );
  }

  const shippingOptions = shippingData?.shippingMethods || [];

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center">
          <Truck className="h-5 w-5 mr-2" />
          配送情報
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <FormInputField
          name="email"
          label="メールアドレス*"
          type="email"
        />

        <div className="grid grid-cols-2 gap-4">
          <FormInputField name="lastName" label="姓*" />
          <FormInputField name="firstName" label="名*" />
        </div>

        <div className="grid grid-cols-3 gap-4">
          <FormInputField name="postalCode" label="郵便番号*" />
          <FormInputField name="prefecture" label="都道府県*" />
          <FormInputField name="city" label="市区町村*" />
        </div>

        <FormInputField name="address" label="番地*" />
        <FormInputField name="apartment" label="アパート・マンション名（任意）" />
        <FormInputField name="phone" label="電話番号*" type="tel" />

        <FormRadioGroup
          name="shippingMethod"
          label="配送方法"
          options={shippingOptions.map(option => ({
            id: option.id,
            label: option.name,
            description: `${option.description} - ¥${option.price.toLocaleString()}`
          }))}
        />

        <Button
          className="w-full"
          onClick={onNext}
        >
          支払い情報へ進む
        </Button>
      </CardContent>
    </Card>
  );
} 