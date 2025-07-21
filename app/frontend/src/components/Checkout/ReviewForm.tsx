import React from "react";
import { useFormContext, Controller } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Textarea } from "@/components/ui/textarea";
import { Shield, AlertCircle, X } from "lucide-react";
import { useGetShippingMethodListQuery } from "@/store/api";
import { useGetPaymentMethodListQuery } from "@/store/generatedApi/paymentApi";
import { type CheckoutFormData } from "@/lib/validators/checkout";

interface ReviewFormProps {
  onBack: () => void;
  total?: number;
  isLoadingTotal?: boolean;
  hasCalculationError?: boolean;
  isCreatingOrder?: boolean;
  orderError?: string | null;
  onClearError?: () => void;
}

export function ReviewForm({
  onBack,
  total = 0,
  isLoadingTotal = false,
  hasCalculationError,
  isCreatingOrder = false,
  orderError,
  onClearError,
}: ReviewFormProps) {
  const {
    register,
    control,
    getValues,
  } = useFormContext<CheckoutFormData>();
  const { data: shippingData } = useGetShippingMethodListQuery();
  const { data: paymentData } = useGetPaymentMethodListQuery();
  const values = getValues();

  const selectedShipping = shippingData?.shippingMethods.find(
    (option) => option.id === values.shippingMethod
  );
  const selectedPayment = paymentData?.items?.find(
    (option) => option.id === values.paymentMethod
  );

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
            <p>
              {values.lastName} {values.firstName}
            </p>
            <p>{values.email}</p>
            <p>〒{values.postalCode}</p>
            <p>
              {values.prefecture} {values.city}
            </p>
            <p>{values.address}</p>
            {values.apartment && <p>{values.apartment}</p>}
            <p>{values.phone}</p>
          </div>
        </div>

        {/* 配送方法 */}
        <div>
          <h3 className="font-medium mb-2">配送方法</h3>
          <p className="text-sm text-muted-foreground">
            {selectedShipping?.name && selectedShipping?.description
              ? `${selectedShipping.name} - ${selectedShipping.description}`
              : "未選択"}
          </p>
        </div>

        {/* 支払い方法 */}
        <div>
          <h3 className="font-medium mb-2">支払い方法</h3>
          <p className="text-sm text-muted-foreground">
            {selectedPayment?.name ? selectedPayment.name : "未選択"}
          </p>
        </div>

        <Textarea
          id="notes"
          label="注文メモ（任意）"
          placeholder="配送に関するご要望などがございましたらご記入ください"
          className="min-h-[80px]"
          {...register("notes")}
        />

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

        {hasCalculationError && (
          <div className="bg-red-50 border border-red-200 rounded-md p-3">
            <div className="flex items-start">
              <AlertCircle className="h-5 w-5 text-red-500 mt-0.5 mr-2 flex-shrink-0" />
              <p className="text-red-800 text-sm">
                合計金額の計算に失敗しました。配送方法と支払い方法を再選択してください。
              </p>
            </div>
          </div>
        )}

        {orderError && (
          <div className="bg-red-50 border border-red-200 rounded-md p-3">
            <div className="flex items-start">
              <AlertCircle className="h-5 w-5 text-red-500 mt-0.5 mr-2 flex-shrink-0" />
              <div className="flex-1">
                <p className="text-red-800 text-sm whitespace-pre-line">{orderError}</p>
              </div>
              {onClearError && (
                <button
                  type="button"
                  onClick={onClearError}
                  className="ml-2 text-red-500 hover:text-red-700"
                >
                  <X className="h-4 w-4" />
                </button>
              )}
            </div>
          </div>
        )}

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
            disabled={isLoadingTotal || hasCalculationError || isCreatingOrder || Boolean(orderError)}
          >
            {isCreatingOrder
              ? "注文処理中..."
              : isLoadingTotal
                  ? "計算中..."
                  : hasCalculationError
                    ? "計算エラー"
                    : orderError
                      ? "エラーを確認してください"
                      : `¥${total.toLocaleString()}で注文確定`}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
