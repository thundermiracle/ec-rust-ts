import { z } from 'zod';

// 配送先情報のスキーマ
export const shippingSchema = z.object({
  email: z.string().email({ message: '有効なメールアドレスを入力してください。' }),
  firstName: z.string().min(1, { message: '名を入力してください。' }),
  lastName: z.string().min(1, { message: '姓を入力してください。' }),
  address: z.string().min(1, { message: '住所を入力してください。' }),
  apartment: z.string().optional(),
  city: z.string().min(1, { message: '市区町村を入力してください。' }),
  postalCode: z.string().regex(/^[0-9]{3}-?[0-9]{4}$/, { message: '有効な郵便番号を入力してください。' }),
  prefecture: z.string().min(1, { message: '都道府県を入力してください。' }),
  phone: z.string().regex(/^0\d{1,4}-?\d{1,4}-?\d{4}$/, { message: '有効な電話番号を入力してください。' }),
  shippingMethod: z.enum(['standard', 'express'], { errorMap: () => ({ message: '配送方法を選択してください。' }) }),
});

// 支払い情報のスキーマ
export const paymentSchema = z.object({
  paymentMethod: z.enum(['credit', 'konbini', 'bank'], { errorMap: () => ({ message: '支払い方法を選択してください。' }) }),
  cardNumber: z.string().optional(),
  expiryDate: z.string().optional(),
  cvc: z.string().optional(),
  cardName: z.string().optional(),
  saveInfo: z.boolean().optional(),
});

// レビュー情報のスキーマ
export const reviewSchema = z.object({
    notes: z.string().optional(),
    subscribeNewsletter: z.boolean().optional(),
});

// チェックアウトフォーム全体のスキーマ
export const checkoutSchema = z.object({
    ...shippingSchema.shape,
    ...paymentSchema.shape,
    ...reviewSchema.shape,
}).refine(data => {
    if (data.paymentMethod === 'credit') {
        return data.cardNumber && data.cardNumber.trim() !== '' &&
               data.expiryDate && data.expiryDate.trim() !== '' &&
               data.cvc && data.cvc.trim() !== '' &&
               data.cardName && data.cardName.trim() !== '';
    }
    return true;
}, {
    message: 'クレジットカード情報をすべて入力してください。',
    path: ['cardNumber'], // or a more general path if appropriate
});

export type ShippingFormData = z.infer<typeof shippingSchema>;
export type PaymentFormData = z.infer<typeof paymentSchema>;
export type ReviewFormData = z.infer<typeof reviewSchema>;
export type CheckoutFormData = z.infer<typeof checkoutSchema>; 