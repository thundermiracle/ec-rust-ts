import React from 'react';
import { useFormContext, Controller } from 'react-hook-form';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { CreditCard } from 'lucide-react';
import { useGetPaymentMethodListQuery } from '@/store/api';
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
  
  // Fetch payment methods from API
  const { data: paymentMethods, isLoading, isError } = useGetPaymentMethodListQuery();

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center">
          <CreditCard className="h-5 w-5 mr-2" />
          支払い情報
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {isLoading ? (
          <div className="text-center py-4">支払い方法を読み込み中...</div>
        ) : isError ? (
          <div className="text-center py-4 text-red-600">支払い方法の読み込みに失敗しました</div>
        ) : (
          <FormRadioGroup
            name="paymentMethod"
            label="支払い方法"
            options={paymentMethods?.items?.map(option => ({
              id: option.id,
              label: option.name || '',
              description: option.description || ''
            })) || []}
          />
        )}

        {/* クレジットカード情報 */}
        {paymentMethod === 'credit_card' && (
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

        {/* 代引き情報 */}
        {paymentMethod === 'cod' && (
          <div className="space-y-4 p-4 bg-muted/30 rounded-lg">
            <div className="flex items-start space-x-3">
              <div className="w-5 h-5 bg-blue-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <span className="text-white text-xs font-bold">!</span>
              </div>
              <div className="space-y-2">
                <h4 className="font-medium text-sm">代引きについて</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• 商品到着時に配送業者へ現金でお支払いください</li>
                  <li>• 代引き手数料: ¥300〜（税込）</li>
                  <li>• お釣りのないようご準備をお願いします</li>
                  <li>• 配達時にご不在の場合は再配達となります</li>
                </ul>
              </div>
            </div>
          </div>
        )}

        {/* 銀行振込情報 */}
        {paymentMethod === 'bank_transfer' && (
          <div className="space-y-4 p-4 bg-muted/30 rounded-lg">
            <div className="flex items-start space-x-3">
              <div className="w-5 h-5 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <span className="text-white text-xs font-bold">¥</span>
              </div>
              <div className="space-y-2">
                <h4 className="font-medium text-sm">銀行振込について</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• ご注文確定後、振込先口座をご案内いたします</li>
                  <li>• 入金確認後に商品を発送いたします</li>
                  <li>• 振込手数料はお客様負担となります</li>
                  <li>• 3営業日以内にお振込みください</li>
                </ul>
              </div>
            </div>
          </div>
        )}

        {/* コンビニ支払い情報 */}
        {paymentMethod === 'convenience_store' && (
          <div className="space-y-4 p-4 bg-muted/30 rounded-lg">
            <div className="flex items-start space-x-3">
              <div className="w-5 h-5 bg-orange-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <span className="text-white text-xs font-bold">C</span>
              </div>
              <div className="space-y-2">
                <h4 className="font-medium text-sm">コンビニ支払いについて</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• セブンイレブン、ファミリーマート、ローソンでお支払い可能</li>
                  <li>• 手数料: ¥200（税込）</li>
                  <li>• お支払い用番号をメールでお送りします</li>
                  <li>• 支払い期限: 注文から7日以内</li>
                </ul>
              </div>
            </div>
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