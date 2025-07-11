'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useForm, FormProvider } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { 
  selectCartItems, 
  clearCart,
} from '@/store/cartSlice';
import { Button } from '@/components/ui/button';
import { Stepper } from '@/components/ui/stepper';
import { 
  ShippingForm, 
  PaymentForm, 
  ReviewForm, 
  OrderSummary,
} from '@/components/Checkout';
import { checkoutSchema, type CheckoutFormData, shippingSchema, paymentSchema } from '@/lib/validators/checkout';
import { ArrowLeft } from 'lucide-react';

const CHECKOUT_STEPS = [
  { id: 'shipping', label: '配送情報' },
  { id: 'payment', label: '支払い' },
  { id: 'review', label: '確認' }
];

export default function CheckoutPage() {
  const router = useRouter();
  const dispatch = useAppDispatch();
  const cartItems = useAppSelector(selectCartItems);
  
  const [currentStep, setCurrentStep] = useState<'shipping' | 'payment' | 'review'>('shipping');
  
  const formContext = useForm<CheckoutFormData>({
    resolver: zodResolver(checkoutSchema),
    defaultValues: {
      email: '',
      firstName: '',
      lastName: '',
      address: '',
      apartment: '',
      city: '',
      postalCode: '',
      prefecture: '',
      phone: '',
      shippingMethod: 'standard',
      paymentMethod: 'credit_card',
      cardNumber: '',
      expiryDate: '',
      cvc: '',
      cardName: '',
      saveInfo: false,
      notes: '',
      subscribeNewsletter: false,
    },
  });

  // カートの初期化が完了してから、カートが空の場合は/に戻る
  useEffect(() => {
    if (cartItems.length === 0) {
      router.push('/');
    }
  }, [cartItems.length, router]);

  // 注文処理
  const handlePlaceOrder = async (data: CheckoutFormData) => {
    try {
      // TODO: 実際のAPIエンドポイントに注文データを送信
      console.log('注文データ:', data);
      
      // 注文完了後、カートをクリアして完了ページへ
      dispatch(clearCart());
      router.push('/checkout/success');
      
    } catch (error) {
      console.error('注文処理エラー:', error);
      alert('注文処理中にエラーが発生しました。再度お試しください。');
    }
  };
  
  const handleNextStep = async () => {
    let isValid = false;
    if (currentStep === 'shipping') {
      const fields = Object.keys(shippingSchema.shape) as (keyof CheckoutFormData)[];
      isValid = await formContext.trigger(fields);
      if (isValid) setCurrentStep('payment');
    } else if (currentStep === 'payment') {
      const fields = Object.keys(paymentSchema.shape) as (keyof CheckoutFormData)[];
      isValid = await formContext.trigger(fields);
      if (isValid) setCurrentStep('review');
    }
  };

  const handlePrevStep = () => {
    if (currentStep === 'review') {
      setCurrentStep('payment');
    } else if (currentStep === 'payment') {
      setCurrentStep('shipping');
    }
  };

  // カートの初期化が完了していない場合はローディング表示
  if (cartItems.length === 0) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center">
          <p className="text-muted-foreground">読み込み中...</p>
        </div>
      </div>
    );
  }

  // カートが空の場合はリダイレクト処理でnullを返す
  if (cartItems.length === 0) {
    return null;
  }

  return (
    <FormProvider {...formContext}>
      <form onSubmit={formContext.handleSubmit(handlePlaceOrder)}>
        <div className="min-h-screen bg-background">
          <header className="py-4 border-b">
            <div className="container mx-auto px-4 flex justify-between items-center">
              <Link href="/" className="text-2xl font-bold">
                EC-SITE
              </Link>
              <Button variant="ghost" size="sm" onClick={() => router.back()}>
                <ArrowLeft className="h-4 w-4 mr-2" />
                買い物を続ける
              </Button>
            </div>
          </header>

          <main className="container mx-auto px-4 py-8">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
              <div className="lg:col-span-1">
                <Stepper
                  steps={CHECKOUT_STEPS}
                  currentStep={currentStep}
                  className="mb-8"
                />

                {currentStep === 'shipping' && (
                  <ShippingForm onNext={handleNextStep} />
                )}
                {currentStep === 'payment' && (
                  <PaymentForm onNext={handleNextStep} onPrev={handlePrevStep} />
                )}
                {currentStep === 'review' && (
                  <ReviewForm onBack={handlePrevStep} />
                )}
              </div>

              <aside className="lg:col-span-1">
                <OrderSummary />
              </aside>
            </div>
          </main>
        </div>
      </form>
    </FormProvider>
  );
} 