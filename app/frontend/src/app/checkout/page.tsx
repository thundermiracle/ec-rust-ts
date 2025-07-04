'use client';

import { useState, useMemo, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { useRouter } from 'next/navigation';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { 
  selectCartItems, 
  clearCart,
  initializeCart
} from '@/store/cartSlice';
import { useGetProductListQuery } from '@/store/generatedApi/productsApi';
import { useFindVariantsMutation } from '@/store/generatedApi/variantsApi';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Separator } from '@/components/ui/separator';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { ArrowLeft, CreditCard, Truck, Shield } from 'lucide-react';
import { enhanceCartItemsWithVariantAPI, calculateCartTotal } from '@/components/Cart/CartDrawer/helper';

// フォームデータの型定義
interface CheckoutFormData {
  // 配送情報
  email: string;
  firstName: string;
  lastName: string;
  address: string;
  apartment?: string;
  city: string;
  postalCode: string;
  prefecture: string;
  phone: string;
  
  // 配送オプション
  shippingMethod: 'standard' | 'express' | 'overnight';
  
  // 支払い情報
  paymentMethod: 'credit' | 'convenience' | 'bank';
  cardNumber: string;
  expiryDate: string;
  cvc: string;
  cardName: string;
  
  // その他
  notes?: string;
  saveInfo: boolean;
  subscribeNewsletter: boolean;
}

const SHIPPING_OPTIONS = [
  {
    id: 'standard' as const,
    name: '標準配送',
    description: '5-7営業日',
    price: 500,
  },
  {
    id: 'express' as const,
    name: '速達配送',
    description: '2-3営業日', 
    price: 1000,
  },
  {
    id: 'overnight' as const,
    name: '翌日配送',
    description: '翌営業日',
    price: 2000,
  },
];

const PAYMENT_OPTIONS = [
  {
    id: 'credit' as const,
    name: 'クレジットカード',
    description: 'Visa, Mastercard, JCB',
  },
  {
    id: 'convenience' as const,
    name: 'コンビニ決済',
    description: 'セブンイレブン、ファミマ、ローソン',
  },
  {
    id: 'bank' as const,
    name: '銀行振込',
    description: '3営業日以内にお振込ください',
  },
];

export default function CheckoutPage() {
  const router = useRouter();
  const dispatch = useAppDispatch();
  const cartItems = useAppSelector(selectCartItems);
  
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentStep, setCurrentStep] = useState<'shipping' | 'payment' | 'review'>('shipping');
  const [isCartInitialized, setIsCartInitialized] = useState(false);
  
  // フォームデータの状態管理
  const [formData, setFormData] = useState<CheckoutFormData>({
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
    paymentMethod: 'credit',
    cardNumber: '',
    expiryDate: '',
    cvc: '',
    cardName: '',
    notes: '',
    saveInfo: false,
    subscribeNewsletter: false,
  });

  // 商品とバリアント情報の取得
  const { data: productListData, isLoading: isProductListLoading } = useGetProductListQuery();
  const [findVariants, { data: variantsData, isLoading: isVariantsLoading }] = useFindVariantsMutation();

  // SKU IDsをソートした文字列として管理
  const skuIdsKey = useMemo(() => {
    const uniqueIds = [...new Set(cartItems.map(item => item.skuId))].sort();
    return uniqueIds.join(',');
  }, [cartItems]);

  // バリアント詳細を取得
  useEffect(() => {
    if (skuIdsKey) {
      const skuIds = skuIdsKey.split(',').filter(Boolean);
      if (skuIds.length > 0) {
        findVariants({ findVariantsRequest: { skuIds } });
      }
    }
  }, [skuIdsKey, findVariants]);

  // 商品データとバリアントデータを結合
  const enhancedCartItems = useMemo(() => {
    if (!productListData?.products || !variantsData?.variants) return [];
    return enhanceCartItemsWithVariantAPI(
      cartItems, 
      productListData.products, 
      variantsData.variants
    );
  }, [cartItems, productListData, variantsData]);

  // 合計金額計算
  const subtotal = useMemo(() => calculateCartTotal(enhancedCartItems), [enhancedCartItems]);
  const selectedShipping = SHIPPING_OPTIONS.find(option => option.id === formData.shippingMethod);
  const shippingCost = selectedShipping?.price || 0;
  const tax = Math.floor((subtotal + shippingCost) * 0.1); // 10%税込
  const total = subtotal + shippingCost + tax;

  // カートを初期化
  useEffect(() => {
    dispatch(initializeCart());
    // 初期化後に少し待ってからチェック
    const timer = setTimeout(() => {
      setIsCartInitialized(true);
    }, 100);
    
    return () => clearTimeout(timer);
  }, [dispatch]);

  // カートの初期化が完了してから、カートが空の場合は/に戻る
  useEffect(() => {
    if (isCartInitialized && cartItems.length === 0) {
      router.push('/');
    }
  }, [isCartInitialized, cartItems.length, router]);

  // フォーム入力の処理
  const handleInputChange = (field: keyof CheckoutFormData, value: string | boolean) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  // 注文処理
  const handlePlaceOrder = async () => {
    setIsSubmitting(true);
    
    try {
      // 注文データを作成
      const orderData = {
        items: enhancedCartItems.map(item => ({
          productId: item.productId,
          skuId: item.skuId,
          quantity: item.quantity,
          price: item.salePrice || item.price,
        })),
        shipping: {
          email: formData.email,
          firstName: formData.firstName,
          lastName: formData.lastName,
          address: formData.address,
          apartment: formData.apartment,
          city: formData.city,
          postalCode: formData.postalCode,
          prefecture: formData.prefecture,
          phone: formData.phone,
          method: formData.shippingMethod,
        },
        payment: {
          method: formData.paymentMethod,
        },
        totals: {
          subtotal,
          shipping: shippingCost,
          tax,
          total,
        },
        notes: formData.notes,
      };

      // 模擬的な処理（2秒待機）
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // 注文完了後、カートをクリアして完了ページへ
      dispatch(clearCart());
      router.push('/checkout/success');
      
    } catch (error) {
      console.error('注文処理エラー:', error);
      alert('注文処理中にエラーが発生しました。再度お試しください。');
    } finally {
      setIsSubmitting(false);
    }
  };

  // 配送情報フォームの検証
  const isShippingValid = () => {
    return formData.email && formData.firstName && formData.lastName &&
           formData.address && formData.city && formData.postalCode && 
           formData.prefecture && formData.phone;
  };

  // 支払い情報フォームの検証（クレジットカードの場合のみ）
  const isPaymentValid = () => {
    if (formData.paymentMethod !== 'credit') return true;
    return formData.cardNumber && formData.expiryDate && formData.cvc && formData.cardName;
  };

  const isLoading = isProductListLoading || isVariantsLoading;

  // カートの初期化が完了していない場合はローディング表示
  if (!isCartInitialized) {
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
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* ヘッダー */}
        <div className="mb-8">
          <Button variant="ghost" asChild className="mb-4">
            <Link href="/">
              <ArrowLeft className="h-4 w-4 mr-2" />
              ショッピングを続ける
            </Link>
          </Button>
          <h1 className="text-3xl font-bold">チェックアウト</h1>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
          {/* 左側：フォーム */}
          <div className="space-y-8">
            {/* ステップインジケータ */}
            <div className="flex items-center space-x-4">
              <div className={`flex items-center ${currentStep === 'shipping' ? 'text-primary' : 'text-muted-foreground'}`}>
                <div className={`w-8 h-8 rounded-full border-2 flex items-center justify-center text-sm font-medium ${
                  currentStep === 'shipping' ? 'border-primary bg-primary text-primary-foreground' : 'border-muted-foreground'
                }`}>
                  1
                </div>
                <span className="ml-2">配送情報</span>
              </div>
              <div className="flex-1 border-t border-muted-foreground" />
              <div className={`flex items-center ${currentStep === 'payment' ? 'text-primary' : 'text-muted-foreground'}`}>
                <div className={`w-8 h-8 rounded-full border-2 flex items-center justify-center text-sm font-medium ${
                  currentStep === 'payment' ? 'border-primary bg-primary text-primary-foreground' : 'border-muted-foreground'
                }`}>
                  2
                </div>
                <span className="ml-2">支払い</span>
              </div>
              <div className="flex-1 border-t border-muted-foreground" />
              <div className={`flex items-center ${currentStep === 'review' ? 'text-primary' : 'text-muted-foreground'}`}>
                <div className={`w-8 h-8 rounded-full border-2 flex items-center justify-center text-sm font-medium ${
                  currentStep === 'review' ? 'border-primary bg-primary text-primary-foreground' : 'border-muted-foreground'
                }`}>
                  3
                </div>
                <span className="ml-2">確認</span>
              </div>
            </div>

            {/* 配送情報フォーム */}
            {currentStep === 'shipping' && (
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center">
                    <Truck className="h-5 w-5 mr-2" />
                    配送情報
                  </CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div>
                    <label htmlFor="email" className="block text-sm font-medium mb-2">メールアドレス*</label>
                    <Input
                      id="email"
                      type="email"
                      value={formData.email}
                      onChange={(e) => handleInputChange('email', e.target.value)}
                      required
                    />
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <label htmlFor="lastName" className="block text-sm font-medium mb-2">姓*</label>
                      <Input
                        id="lastName"
                        value={formData.lastName}
                        onChange={(e) => handleInputChange('lastName', e.target.value)}
                        required
                      />
                    </div>
                    <div>
                      <label htmlFor="firstName" className="block text-sm font-medium mb-2">名前*</label>
                      <Input
                        id="firstName"
                        value={formData.firstName}
                        onChange={(e) => handleInputChange('firstName', e.target.value)}
                        required
                      />
                    </div>
                  </div>

                  <div>
                    <label htmlFor="address" className="block text-sm font-medium mb-2">住所*</label>
                    <Input
                      id="address"
                      value={formData.address}
                      onChange={(e) => handleInputChange('address', e.target.value)}
                      required
                    />
                  </div>

                  <div>
                    <label htmlFor="apartment" className="block text-sm font-medium mb-2">アパート・マンション名（任意）</label>
                    <Input
                      id="apartment"
                      value={formData.apartment || ''}
                      onChange={(e) => handleInputChange('apartment', e.target.value)}
                    />
                  </div>

                  <div className="grid grid-cols-3 gap-4">
                    <div>
                      <label htmlFor="city" className="block text-sm font-medium mb-2">市区町村*</label>
                      <Input
                        id="city"
                        value={formData.city}
                        onChange={(e) => handleInputChange('city', e.target.value)}
                        required
                      />
                    </div>
                    <div>
                      <label htmlFor="prefecture" className="block text-sm font-medium mb-2">都道府県*</label>
                      <Input
                        id="prefecture"
                        value={formData.prefecture}
                        onChange={(e) => handleInputChange('prefecture', e.target.value)}
                        required
                      />
                    </div>
                    <div>
                      <label htmlFor="postalCode" className="block text-sm font-medium mb-2">郵便番号*</label>
                      <Input
                        id="postalCode"
                        value={formData.postalCode}
                        onChange={(e) => handleInputChange('postalCode', e.target.value)}
                        required
                      />
                    </div>
                  </div>

                  <div>
                    <label htmlFor="phone" className="block text-sm font-medium mb-2">電話番号*</label>
                    <Input
                      id="phone"
                      type="tel"
                      value={formData.phone}
                      onChange={(e) => handleInputChange('phone', e.target.value)}
                      required
                    />
                  </div>

                  {/* 配送方法選択 */}
                  <div>
                    <label className="block text-sm font-medium mb-2">配送方法</label>
                    <div className="space-y-2">
                      {SHIPPING_OPTIONS.map((option) => (
                        <div key={option.id} className="flex items-center space-x-2 border rounded p-3">
                          <input
                            type="radio"
                            id={option.id}
                            name="shippingMethod"
                            value={option.id}
                            checked={formData.shippingMethod === option.id}
                            onChange={() => handleInputChange('shippingMethod', option.id)}
                            className="h-4 w-4"
                          />
                          <label htmlFor={option.id} className="flex-1 cursor-pointer">
                            <div className="flex justify-between">
                              <div>
                                <div className="font-medium">{option.name}</div>
                                <div className="text-sm text-muted-foreground">{option.description}</div>
                              </div>
                              <div className="font-medium">¥{option.price.toLocaleString()}</div>
                            </div>
                          </label>
                        </div>
                      ))}
                    </div>
                  </div>

                  <Button 
                    className="w-full" 
                    onClick={() => setCurrentStep('payment')}
                    disabled={!isShippingValid()}
                  >
                    支払い情報へ進む
                  </Button>
                </CardContent>
              </Card>
            )}

            {/* 支払い情報フォーム */}
            {currentStep === 'payment' && (
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center">
                    <CreditCard className="h-5 w-5 mr-2" />
                    支払い情報
                  </CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium mb-2">支払い方法</label>
                    <div className="space-y-2">
                      {PAYMENT_OPTIONS.map((option) => (
                        <div key={option.id} className="flex items-center space-x-2 border rounded p-3">
                          <input
                            type="radio"
                            id={option.id}
                            name="paymentMethod"
                            value={option.id}
                            checked={formData.paymentMethod === option.id}
                            onChange={() => handleInputChange('paymentMethod', option.id)}
                            className="h-4 w-4"
                          />
                          <label htmlFor={option.id} className="flex-1 cursor-pointer">
                            <div className="font-medium">{option.name}</div>
                            <div className="text-sm text-muted-foreground">{option.description}</div>
                          </label>
                        </div>
                      ))}
                    </div>
                  </div>

                  {/* クレジットカード情報 */}
                  {formData.paymentMethod === 'credit' && (
                    <div className="space-y-4">
                      <div>
                        <label htmlFor="cardNumber" className="block text-sm font-medium mb-2">カード番号*</label>
                        <Input
                          id="cardNumber"
                          placeholder="1234 5678 9012 3456"
                          value={formData.cardNumber}
                          onChange={(e) => handleInputChange('cardNumber', e.target.value)}
                          required
                        />
                      </div>

                      <div className="grid grid-cols-2 gap-4">
                        <div>
                          <label htmlFor="expiryDate" className="block text-sm font-medium mb-2">有効期限*</label>
                          <Input
                            id="expiryDate"
                            placeholder="MM/YY"
                            value={formData.expiryDate}
                            onChange={(e) => handleInputChange('expiryDate', e.target.value)}
                            required
                          />
                        </div>
                        <div>
                          <label htmlFor="cvc" className="block text-sm font-medium mb-2">CVC*</label>
                          <Input
                            id="cvc"
                            placeholder="123"
                            value={formData.cvc}
                            onChange={(e) => handleInputChange('cvc', e.target.value)}
                            required
                          />
                        </div>
                      </div>

                      <div>
                        <label htmlFor="cardName" className="block text-sm font-medium mb-2">カード名義*</label>
                        <Input
                          id="cardName"
                          placeholder="YAMADA TARO"
                          value={formData.cardName}
                          onChange={(e) => handleInputChange('cardName', e.target.value)}
                          required
                        />
                      </div>
                    </div>
                  )}

                  <div className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      id="saveInfo"
                      checked={formData.saveInfo}
                      onChange={(e) => handleInputChange('saveInfo', e.target.checked)}
                      className="h-4 w-4"
                    />
                    <label htmlFor="saveInfo" className="text-sm">
                      次回のために情報を保存する
                    </label>
                  </div>

                  <div className="flex space-x-4">
                    <Button 
                      variant="outline" 
                      className="flex-1"
                      onClick={() => setCurrentStep('shipping')}
                    >
                      戻る
                    </Button>
                    <Button 
                      className="flex-1"
                      onClick={() => setCurrentStep('review')}
                      disabled={!isPaymentValid()}
                    >
                      注文内容を確認
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}

            {/* 注文確認 */}
            {currentStep === 'review' && (
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
                      <p>{formData.lastName} {formData.firstName}</p>
                      <p>{formData.email}</p>
                      <p>〒{formData.postalCode}</p>
                      <p>{formData.prefecture} {formData.city}</p>
                      <p>{formData.address}</p>
                      {formData.apartment && <p>{formData.apartment}</p>}
                      <p>{formData.phone}</p>
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
                      {PAYMENT_OPTIONS.find(option => option.id === formData.paymentMethod)?.name}
                    </p>
                  </div>

                  {/* 注文メモ */}
                  <div>
                    <label htmlFor="notes" className="block text-sm font-medium mb-2">注文メモ（任意）</label>
                    <textarea
                      id="notes"
                      placeholder="配送に関するご要望などがございましたらご記入ください"
                      value={formData.notes || ''}
                      onChange={(e) => handleInputChange('notes', e.target.value)}
                      className="w-full min-h-[80px] rounded-md border border-input bg-background px-3 py-2 text-sm"
                    />
                  </div>

                  <div className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      id="newsletter"
                      checked={formData.subscribeNewsletter}
                      onChange={(e) => handleInputChange('subscribeNewsletter', e.target.checked)}
                      className="h-4 w-4"
                    />
                    <label htmlFor="newsletter" className="text-sm">
                      ニュースレターを購読する
                    </label>
                  </div>

                  <div className="flex space-x-4">
                    <Button 
                      variant="outline" 
                      className="flex-1"
                      onClick={() => setCurrentStep('payment')}
                    >
                      戻る
                    </Button>
                    <Button 
                      className="flex-1"
                      onClick={handlePlaceOrder}
                      disabled={isSubmitting}
                    >
                      {isSubmitting ? '注文処理中...' : `¥${total.toLocaleString()}で注文確定`}
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}
          </div>

          {/* 右側：注文概要 */}
          <div>
            <Card className="sticky top-8">
              <CardHeader>
                <CardTitle>注文概要</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                {isLoading ? (
                  <div className="text-center py-8">
                    <p className="text-muted-foreground">読み込み中...</p>
                  </div>
                ) : (
                  <>
                    {/* 商品リスト */}
                    <div className="space-y-4">
                      {enhancedCartItems.map((item) => (
                        <div key={`${item.productId}-${item.skuId}`} className="flex items-center space-x-3">
                          <div className="relative w-16 h-16 flex-shrink-0">
                            <Image
                              src={item.image || '/images/placeholder.jpg'}
                              alt={item.name || 'Product'}
                              fill
                              className="object-cover rounded"
                            />
                            <div className="absolute -top-2 -right-2 bg-primary text-primary-foreground text-xs rounded-full h-5 w-5 flex items-center justify-center">
                              {item.quantity}
                            </div>
                          </div>
                          <div className="flex-1 min-w-0">
                            <h3 className="font-medium text-sm truncate">{item.name}</h3>
                            {item.material && (
                              <p className="text-xs text-muted-foreground">Material: {item.material}</p>
                            )}
                            <div className="flex items-center justify-between mt-1">
                              <span className="text-sm text-muted-foreground">
                                数量: {item.quantity}
                              </span>
                              <span className="font-medium text-sm">
                                ¥{((item.salePrice || item.price || 0) * item.quantity).toLocaleString()}
                              </span>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>

                    <Separator />

                    {/* 金額詳細 */}
                    <div className="space-y-2">
                      <div className="flex justify-between text-sm">
                        <span>小計</span>
                        <span>¥{subtotal.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span>配送料</span>
                        <span>¥{shippingCost.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span>税込</span>
                        <span>¥{tax.toLocaleString()}</span>
                      </div>
                      <Separator />
                      <div className="flex justify-between font-bold">
                        <span>合計</span>
                        <span>¥{total.toLocaleString()}</span>
                      </div>
                    </div>
                  </>
                )}
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
} 