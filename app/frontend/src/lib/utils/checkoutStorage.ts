import { type CheckoutFormData } from '@/lib/validators/checkout';

const STORAGE_KEY = 'checkout_form_data';

// localStorageに保存する際に除外するフィールド（セキュリティ上機密情報）
const SENSITIVE_FIELDS = ['cardNumber', 'cvc', 'expiryDate'] as const;

// セキュリティ上保存しないフィールドを除外した型
export type SafeCheckoutData = Omit<CheckoutFormData, typeof SENSITIVE_FIELDS[number]>;

/**
 * チェックアウト情報をlocalStorageに保存する
 * セキュリティのためクレジットカード情報は除外
 */
export function saveCheckoutData(data: Partial<CheckoutFormData>): void {
  try {
    // 機密情報を除外したデータを作成
    const safeData: Partial<SafeCheckoutData> = { ...data };
    
    // 機密フィールドを削除
    SENSITIVE_FIELDS.forEach(field => {
      delete (safeData as Record<string, unknown>)[field];
    });

    localStorage.setItem(STORAGE_KEY, JSON.stringify(safeData));
  } catch (error) {
    console.warn('チェックアウト情報の保存に失敗しました:', error);
  }
}

/**
 * localStorageからチェックアウト情報を復元する
 */
export function loadCheckoutData(): Partial<SafeCheckoutData> | null {
  try {
    const storedData = localStorage.getItem(STORAGE_KEY);
    if (!storedData) return null;

    return JSON.parse(storedData);
  } catch (error) {
    console.warn('チェックアウト情報の読み込みに失敗しました:', error);
    return null;
  }
}

/**
 * localStorageからチェックアウト情報を削除する
 */
export function clearCheckoutData(): void {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch (error) {
    console.warn('チェックアウト情報の削除に失敗しました:', error);
  }
}

/**
 * 特定のフィールドのみをlocalStorageに保存する
 */
export function saveCheckoutField<K extends keyof SafeCheckoutData>(
  field: K, 
  value: SafeCheckoutData[K]
): void {
  try {
    const currentData = loadCheckoutData() || {};
    const updatedData = { ...currentData, [field]: value };
    
    localStorage.setItem(STORAGE_KEY, JSON.stringify(updatedData));
  } catch (error) {
    console.warn(`チェックアウトフィールド ${field} の保存に失敗しました:`, error);
  }
}

/**
 * 配送情報のみを保存する
 */
export function saveShippingData(data: Partial<CheckoutFormData>): void {
  const shippingFields = [
    'email', 'firstName', 'lastName', 'address', 'apartment', 
    'city', 'postalCode', 'prefecture', 'phone', 'shippingMethod'
  ] as const;

  const shippingData: Partial<SafeCheckoutData> = {};
  shippingFields.forEach(field => {
    if (data[field] !== undefined) {
      (shippingData as Record<string, unknown>)[field] = data[field];
    }
  });

  try {
    const currentData = loadCheckoutData() || {};
    const updatedData = { ...currentData, ...shippingData };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(updatedData));
  } catch (error) {
    console.warn('配送情報の保存に失敗しました:', error);
  }
}

/**
 * 支払い情報のみを保存する（クレジットカード情報は除外）
 */
export function savePaymentData(data: Partial<CheckoutFormData>): void {
  const paymentFields = ['paymentMethod', 'cardName', 'saveInfo'] as const;

  const paymentData: Partial<SafeCheckoutData> = {};
  paymentFields.forEach(field => {
    if (data[field] !== undefined) {
      (paymentData as Record<string, unknown>)[field] = data[field];
    }
  });

  try {
    const currentData = loadCheckoutData() || {};
    const updatedData = { ...currentData, ...paymentData };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(updatedData));
  } catch (error) {
    console.warn('支払い情報の保存に失敗しました:', error);
  }
}