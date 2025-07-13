import { useEffect } from 'react';
import { UseFormReturn } from 'react-hook-form';
import { type CheckoutFormData } from '@/lib/validators/checkout';
import { 
  saveCheckoutData, 
  loadCheckoutData, 
  clearCheckoutData, 
  saveShippingData, 
  savePaymentData 
} from '@/lib/utils/checkoutStorage';

interface UseCheckoutStorageOptions {
  formContext: UseFormReturn<CheckoutFormData>;
  autoSaveDelay?: number;
}

export function useCheckoutStorage({ 
  formContext, 
  autoSaveDelay = 500 
}: UseCheckoutStorageOptions) {
  // localStorageからデータを復元
  useEffect(() => {
    const savedData = loadCheckoutData();
    if (savedData) {
      // 復元されたデータをフォームに設定
      Object.entries(savedData).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          formContext.setValue(key as keyof CheckoutFormData, value);
        }
      });
    }
  }, [formContext]);

  // フォームの値を監視してlocalStorageに自動保存
  const watchedValues = formContext.watch();
  useEffect(() => {
    const timeoutId = setTimeout(() => {
      saveCheckoutData(watchedValues);
    }, autoSaveDelay);

    return () => clearTimeout(timeoutId);
  }, [watchedValues, autoSaveDelay]);

  // 配送情報を保存する関数
  const saveShipping = () => {
    saveShippingData(formContext.getValues());
  };

  // 支払い情報を保存する関数（クレジットカード情報除く）
  const savePayment = () => {
    savePaymentData(formContext.getValues());
  };

  // チェックアウト情報をクリアする関数
  const clearStorage = () => {
    clearCheckoutData();
  };

  // 手動で全体を保存する関数
  const saveAll = () => {
    saveCheckoutData(formContext.getValues());
  };

  return {
    saveShipping,
    savePayment,
    clearStorage,
    saveAll,
  };
}