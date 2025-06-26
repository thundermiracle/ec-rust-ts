'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';
import { useGetProductQuery } from '@/store';
import { Separator } from '@/components/ui/separator';
import { ChevronRight } from 'lucide-react';

// コンポーネントをインポート
import {
  ProductLoadingState,
  ProductErrorState,
  ProductImageGallery,
  ProductInfo,
  ProductVariants,
  ProductColors,
  ProductPurchase,
  ProductDetails
} from '@/components/ProductDetail';

export default function ProductDetail() {
  const params = useParams<{ id: string }>();
  const productId = params.id;
  
  // RTK Queryを使用してプロダクトデータを取得
  const { data: product, isLoading, error } = useGetProductQuery({id: productId});
  
  const [selectedVariantIndex, setSelectedVariantIndex] = useState<number>(0);
  const [quantity, setQuantity] = useState(1);

  // ローディング状態
  if (isLoading) {
    return <ProductLoadingState />;
  }

  // エラー状態
  if (error) {
    return <ProductErrorState />;
  }

  // プロダクトが見つからない場合
  if (!product || !product.variants) {
    return <ProductErrorState isNotFound />;
  }

  const selectedVariant = product.variants[selectedVariantIndex];
  const allImages = product.images.concat([selectedVariant.image ?? '']).filter(Boolean);
  const allColors = product.variants.map((variant) => variant.color);

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Breadcrumb */}
        <nav className="mb-8">
          <div className="flex items-center text-sm text-muted-foreground">
            <Link href="/" className="hover:text-foreground transition-colors">
              Home
            </Link>
            <ChevronRight className="h-4 w-4 mx-2" />
            <span className="text-foreground">{product.name}</span>
          </div>
        </nav>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
          {/* Product Images */}
          <ProductImageGallery
            productName={product.name}
            allImages={allImages}
            selectedVariant={selectedVariant}
            isBestSeller={product.isBestSeller}
            isQuickShip={product.isQuickShip}
          />

          {/* Product Info */}
          <div className="space-y-8">
            <ProductInfo
              name={product.name}
              description={product.description}
              selectedVariant={selectedVariant}
            />

            {/* Variants */}
            <ProductVariants
              variants={product.variants}
              selectedVariant={selectedVariant}
              onVariantChange={setSelectedVariantIndex}
            />

            {/* Colors */}
            <ProductColors colors={allColors} />

            {/* Quantity and Add to Cart */}
            <ProductPurchase
              quantity={quantity}
              onQuantityChange={setQuantity}
              isSoldOut={selectedVariant.isSoldOut}
              productId={product.id}
              skuId={selectedVariant.id}
            />

            <Separator />

            {/* Product Details */}
            <ProductDetails
              selectedVariant={selectedVariant}
              category={product.category}
            />
          </div>
        </div>
      </div>
    </div>
  );
} 