'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';
import Image from 'next/image';
import { ProductVariant } from '../../../types/product';
import { useGetProductQuery } from '../../../lib/api';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Card, CardContent } from '@/components/ui/card';
import { ChevronRight, ImageIcon, Loader2 } from 'lucide-react';

export default function ProductDetail() {
  const params = useParams<{ id: string }>();
  const productId = params.id;
  
  // RTK Queryを使用してプロダクトデータを取得
  const { data: product, isLoading, error } = useGetProductQuery(productId);
  
  const [selectedImage, setSelectedImage] = useState(0);
  const [selectedVariantIndex, setSelectedVariantIndex] = useState<number>(0);
  const [quantity, setQuantity] = useState(1);

  // ローディング状態
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Card className="max-w-md">
          <CardContent className="pt-6 text-center">
            <Loader2 className="h-8 w-8 animate-spin mx-auto mb-4" />
            <h1 className="text-2xl font-bold mb-4">Loading Product...</h1>
            <p className="text-muted-foreground">Please wait while we fetch the product details.</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  // エラー状態
  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Card className="max-w-md">
          <CardContent className="pt-6 text-center">
            <h1 className="text-2xl font-bold mb-4 text-destructive">Error Loading Product</h1>
            <p className="text-muted-foreground mb-4">
                             Sorry, we couldn&apos;t load the product details. Please try again later.
            </p>
            <Button asChild variant="outline">
              <Link href="/">
                Return to All Products
              </Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  // プロダクトが見つからない場合
  if (!product) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Card className="max-w-md">
          <CardContent className="pt-6 text-center">
            <h1 className="text-2xl font-bold mb-4">Product Not Found</h1>
            <p className="text-muted-foreground mb-4">
                             The product you&apos;re looking for doesn&apos;t exist or has been removed.
            </p>
            <Button asChild variant="outline">
              <Link href="/">
                Return to All Products
              </Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const formatPrice = (price: number) => `¥${price.toLocaleString()}`;
  const selectedVariant = product.variants[selectedVariantIndex];
  
  const allImages = product.images.concat([selectedVariant.image ?? '']).filter(Boolean);
  const allColors = product.variants.map((variant: ProductVariant) => variant.color);

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
          <div className="space-y-4">
            {/* Main Image */}
            <Card className="aspect-square relative overflow-hidden border-0 shadow-none bg-muted">
              {allImages[selectedImage] ? (
                <Image
                  src={allImages[selectedImage]}
                  alt={product.name}
                  fill
                  className="object-cover rounded-lg"
                  priority
                />
              ) : (
                <div className="w-full h-full flex items-center justify-center">
                  <ImageIcon className="w-24 h-24 text-muted-foreground/50" />
                </div>
              )}

              {/* Status Badges */}
              <div className="absolute top-4 left-4 flex flex-col gap-2">
                {selectedVariant.isSoldOut && (
                  <Badge variant="destructive">Sold Out</Badge>
                )}
                {selectedVariant.salePrice && (
                  <Badge className="bg-green-600 hover:bg-green-700">
                    -
                    {Math.round(
                      ((selectedVariant.price - selectedVariant.salePrice) /
                        selectedVariant.price) *
                        100
                    )}
                    % OFF
                  </Badge>
                )}
                {product.isBestSeller && (
                  <Badge variant="default">Best Seller</Badge>
                )}
                {product.isQuickShip && (
                  <Badge
                    variant="secondary"
                    className="bg-blue-600 text-white hover:bg-blue-700"
                  >
                    Quick Ship
                  </Badge>
                )}
              </div>
            </Card>

            {/* Thumbnail Images */}
            {product.images.length > 1 && !selectedVariant && (
              <div className="flex gap-4">
                {product.images.map((image: string, index: number) => (
                  <button
                    key={index}
                    onClick={() => setSelectedImage(index)}
                    className={`w-20 h-20 relative bg-gray-50 rounded-lg overflow-hidden border-2 transition-colors ${
                      selectedImage === index
                        ? "border-black"
                        : "border-transparent hover:border-gray-300"
                    }`}
                  >
                    <Image
                      src={image}
                      alt={`${product.name} ${index + 1}`}
                      fill
                      className="object-cover"
                    />
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Product Info */}
          <div className="space-y-8">
            <div>
              <h1 className="text-3xl font-bold text-foreground mb-4">
                {product.name}
              </h1>

              {/* Price */}
              <div className="flex items-center gap-4 mb-6">
                {selectedVariant.salePrice ? (
                  <>
                    <span className="text-2xl font-bold text-foreground">
                      {formatPrice(selectedVariant.salePrice)}
                    </span>
                    <span className="text-xl text-muted-foreground line-through">
                      {formatPrice(selectedVariant.price)}
                    </span>
                    <Badge className="bg-green-100 text-green-800 hover:bg-green-200">
                      Save{" "}
                      {formatPrice(
                        selectedVariant.price - selectedVariant.salePrice
                      )}
                    </Badge>
                  </>
                ) : (
                  <span className="text-2xl font-bold text-foreground">
                    {formatPrice(selectedVariant.price)}
                  </span>
                )}
              </div>

              <p className="text-muted-foreground leading-relaxed">
                {product.description}
              </p>
            </div>

            {/* Variants */}
            {product.variants && product.variants.length > 0 && (
              <div>
                <h3 className="text-lg font-medium text-foreground mb-4">
                  Options
                </h3>
                <div className="space-y-2">
                  {product.variants.map(
                    (variant: ProductVariant, index: number) => (
                      <Card
                        key={variant.id}
                        className={`cursor-pointer transition-colors ${
                          selectedVariant?.id === variant.id
                            ? "border-foreground bg-muted/50"
                            : variant.isSoldOut
                            ? "bg-muted/50 cursor-not-allowed"
                            : "hover:border-muted-foreground"
                        }`}
                        onClick={() =>
                          !variant.isSoldOut && setSelectedVariantIndex(index)
                        }
                      >
                        <CardContent className="p-4">
                          <div className="flex justify-between items-center">
                            <span
                              className={`font-medium ${
                                variant.isSoldOut ? "text-muted-foreground" : ""
                              }`}
                            >
                              {variant.name}
                            </span>
                            <span
                              className={`font-bold ${
                                variant.isSoldOut ? "text-muted-foreground" : ""
                              }`}
                            >
                              {variant.salePrice
                                ? formatPrice(variant.salePrice)
                                : formatPrice(variant.price)}
                            </span>
                          </div>
                          {variant.isSoldOut && (
                            <span className="text-sm text-muted-foreground">
                              Out of stock
                            </span>
                          )}
                        </CardContent>
                      </Card>
                    )
                  )}
                </div>
              </div>
            )}

            {/* Colors */}
            {allColors.length > 0 && (
              <div>
                <h3 className="text-lg font-medium text-foreground mb-4">
                  Available Colors
                </h3>
                <div className="flex gap-3">
                  {allColors.map((color: string, index: number) => (
                    <div
                      key={index}
                      className="flex flex-col items-center gap-2"
                    >
                      <div
                        className="w-10 h-10 rounded-full border-2 border-border shadow-sm"
                        style={{ backgroundColor: getColorValue(color) }}
                        title={color}
                      />
                      <span className="text-xs text-muted-foreground">
                        {color}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Quantity and Add to Cart */}
            <div className="space-y-6">
              <div>
                <label
                  htmlFor="quantity"
                  className="block text-sm font-medium text-foreground mb-2"
                >
                  Quantity
                </label>
                <Select
                  value={quantity.toString()}
                  onValueChange={(value) => setQuantity(Number(value))}
                >
                  <SelectTrigger className="w-24">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {[...Array(10)].map((_, i) => (
                      <SelectItem key={i + 1} value={(i + 1).toString()}>
                        {i + 1}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <Button
                disabled={
                  selectedVariant.isSoldOut
                }
                size="lg"
                className="w-full"
              >
                {selectedVariant.isSoldOut
                  ? "Out of Stock"
                  : "Add to Cart"}
              </Button>
            </div>

            <Separator />

            {/* Product Details */}
            <div>
              <h3 className="text-lg font-medium text-foreground mb-4">
                Product Details
              </h3>
              <div className="space-y-3 text-sm text-muted-foreground">
                {selectedVariant.material && (
                  <div className="flex justify-between">
                    <span>Material:</span>
                    <span className="text-foreground">
                      {selectedVariant.material}
                    </span>
                  </div>
                )}
                {selectedVariant.dimensions && (
                  <div className="flex justify-between">
                    <span>Dimensions:</span>
                    <span className="text-foreground">
                      {selectedVariant.dimensions}
                    </span>
                  </div>
                )}
                <div className="flex justify-between">
                  <span>Category:</span>
                  <span className="text-foreground capitalize">
                    {product.category.replace("-", " ")}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// Helper function to convert color names to CSS values
const getColorValue = (colorName: string): string => {
  const colorMap: { [key: string]: string } = {
    'Walnut': '#8B4513',
    'White Oak': '#F5F5DC',
    'Black Oak': '#2F2F2F',
    'Whitewash Oak': '#F8F8F8',
    'Black': '#000000',
    'White': '#FFFFFF',
    'Charcoal': '#36454F',
    'Mist': '#E6E6FA',
    'Smoke': '#738276',
    'Sand': '#C2B280',
    'Gray': '#808080',
    'Beige': '#F5F5DC',
  };
  
  return colorMap[colorName] || '#CCCCCC';
}; 