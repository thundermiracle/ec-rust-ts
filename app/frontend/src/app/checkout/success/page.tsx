'use client';

import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { CheckCircle, Package, Mail, ArrowRight } from 'lucide-react';

export default function CheckoutSuccessPage() {
  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="max-w-2xl mx-auto text-center">
          {/* 成功メッセージ */}
          <div className="mb-8">
            <CheckCircle className="w-16 h-16 text-green-500 mx-auto mb-4" />
            <h1 className="text-3xl font-bold text-foreground mb-2">ご注文ありがとうございます！</h1>
            <p className="text-muted-foreground">
              ご注文が正常に処理されました。注文確認メールをお送りしております。
            </p>
          </div>

          {/* 注文番号（模擬） */}
          <Card className="mb-8">
            <CardHeader>
              <CardTitle className="flex items-center justify-center">
                <Package className="w-5 h-5 mr-2" />
                注文番号
              </CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-2xl font-mono font-bold">
                #ARX-{Date.now().toString().slice(-8)}
              </p>
              <p className="text-sm text-muted-foreground mt-2">
                この番号は注文の追跡に使用できます
              </p>
            </CardContent>
          </Card>

          {/* 次のステップ */}
          <Card className="mb-8">
            <CardHeader>
              <CardTitle className="flex items-center justify-center">
                <Mail className="w-5 h-5 mr-2" />
                次のステップ
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="text-left space-y-3">
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium flex-shrink-0 mt-0.5">
                    1
                  </div>
                  <div>
                    <p className="font-medium">注文確認メール</p>
                    <p className="text-sm text-muted-foreground">
                      ご注文の詳細を含む確認メールが数分以内に送信されます
                    </p>
                  </div>
                </div>
                
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium flex-shrink-0 mt-0.5">
                    2
                  </div>
                  <div>
                    <p className="font-medium">商品の準備</p>
                    <p className="text-sm text-muted-foreground">
                      1-2営業日以内に商品の準備を開始いたします
                    </p>
                  </div>
                </div>
                
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium flex-shrink-0 mt-0.5">
                    3
                  </div>
                  <div>
                    <p className="font-medium">配送通知</p>
                    <p className="text-sm text-muted-foreground">
                      商品が発送されましたら、追跡番号をお送りします
                    </p>
                  </div>
                </div>
                
                <div className="flex items-start space-x-3">
                  <div className="w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium flex-shrink-0 mt-0.5">
                    4
                  </div>
                  <div>
                    <p className="font-medium">お届け</p>
                    <p className="text-sm text-muted-foreground">
                      選択された配送方法に従って商品をお届けします
                    </p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* アクションボタン */}
          <div className="space-y-4">
            <Button asChild size="lg" className="w-full sm:w-auto">
              <Link href="/">
                ショッピングを続ける
                <ArrowRight className="w-4 h-4 ml-2" />
              </Link>
            </Button>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Button variant="outline" asChild>
                <Link href="/contact">
                  お問い合わせ
                </Link>
              </Button>
              
              <Button variant="outline" asChild>
                <Link href="/about">
                  配送について
                </Link>
              </Button>
            </div>
          </div>

          {/* サポート情報 */}
          <div className="mt-12 p-6 bg-muted rounded-lg">
            <h3 className="font-medium mb-2">ご不明な点がございますか？</h3>
            <p className="text-sm text-muted-foreground mb-4">
              ご注文に関してご質問やご不明な点がございましたら、お気軽にお問い合わせください。
            </p>
            <div className="space-y-2 text-sm">
              <p>
                <span className="font-medium">Email:</span> support@artifox.com
              </p>
              <p>
                <span className="font-medium">電話:</span> 03-1234-5678
              </p>
              <p>
                <span className="font-medium">営業時間:</span> 平日 9:00-18:00
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
} 