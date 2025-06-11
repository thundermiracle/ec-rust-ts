'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';
import Image from 'next/image';
import { products } from '../../../data/mockData';
import { Product, ProductVariant } from '../../../types/product';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Card, CardContent } from '@/components/ui/card';
import { ChevronRight, ImageIcon } from 'lucide-react';

export default function ProductDetail() {
  const params = useParams();
  const productId = params.id as string;
  
  const product = products.find((p: Product) => p.id === productId);
  
  const [selectedImage, setSelectedImage] = useState(0);
  const [selectedVariant, setSelectedVariant] = useState<ProductVariant | null>(
    product?.variants?.[0] || null
  );
  const [quantity, setQuantity] = useState(1);

  if (!product) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Card className="max-w-md">
          <CardContent className="pt-6 text-center">
            <h1 className="text-2xl font-bold mb-4">Product Not Found</h1>
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

  const formatPrice = (price: number) => `$${price.toLocaleString()}`;
  
  const currentPrice = selectedVariant ? selectedVariant.price : product.price;
  const currentSalePrice = selectedVariant ? selectedVariant.salePrice : product.salePrice;
  const currentImage = selectedVariant ? selectedVariant.image : product.images[selectedImage];

  const discountPercentage = currentSalePrice 
    ? Math.round(((currentPrice - currentSalePrice) / currentPrice) * 100)
    : 0;

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
              {currentImage || product.images[selectedImage] ? (
                <Image
                  src={currentImage || product.images[selectedImage]}
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
                {product.isSoldOut && (
                  <Badge variant="destructive">
                    Sold Out
                  </Badge>
                )}
                {product.isOnSale && !product.isSoldOut && (
                  <Badge className="bg-green-600 hover:bg-green-700">
                    -{discountPercentage}% OFF
                  </Badge>
                )}
                {product.isBestSeller && (
                  <Badge variant="default">
                    Best Seller
                  </Badge>
                )}
                {product.isQuickShip && (
                  <Badge variant="secondary" className="bg-blue-600 text-white hover:bg-blue-700">
                    Quick Ship
                  </Badge>
                )}
              </div>
            </Card>

            {/* Thumbnail Images */}
            {product.images.length > 1 && !selectedVariant && (
              <div className="flex gap-4">
                {product.images.map((image, index) => (
                  <button
                    key={index}
                    onClick={() => setSelectedImage(index)}
                    className={`w-20 h-20 relative bg-gray-50 rounded-lg overflow-hidden border-2 transition-colors ${
                      selectedImage === index ? 'border-black' : 'border-transparent hover:border-gray-300'
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
              <h1 className="text-3xl font-bold text-foreground mb-4">{product.name}</h1>
              
              {/* Price */}
              <div className="flex items-center gap-4 mb-6">
                {currentSalePrice ? (
                  <>
                    <span className="text-2xl font-bold text-foreground">
                      {formatPrice(currentSalePrice)}
                    </span>
                    <span className="text-xl text-muted-foreground line-through">
                      {formatPrice(currentPrice)}
                    </span>
                    <Badge className="bg-green-100 text-green-800 hover:bg-green-200">
                      Save {formatPrice(currentPrice - currentSalePrice)}
                    </Badge>
                  </>
                ) : (
                  <span className="text-2xl font-bold text-foreground">
                    {formatPrice(currentPrice)}
                  </span>
                )}
              </div>

              <p className="text-muted-foreground leading-relaxed">{product.description}</p>
            </div>

            {/* Variants */}
            {product.variants && product.variants.length > 0 && (
              <div>
                <h3 className="text-lg font-medium text-foreground mb-4">Options</h3>
                <div className="space-y-2">
                  {product.variants.map((variant) => (
                    <Card
                      key={variant.id}
                      className={`cursor-pointer transition-colors ${
                        selectedVariant?.id === variant.id
                          ? 'border-foreground bg-muted/50'
                          : variant.isAvailable
                          ? 'hover:border-muted-foreground'
                          : 'bg-muted/50 cursor-not-allowed'
                      }`}
                      onClick={() => variant.isAvailable && setSelectedVariant(variant)}
                    >
                      <CardContent className="p-4">
                        <div className="flex justify-between items-center">
                          <span className={`font-medium ${!variant.isAvailable ? 'text-muted-foreground' : ''}`}>
                            {variant.name}
                          </span>
                          <span className={`font-bold ${!variant.isAvailable ? 'text-muted-foreground' : ''}`}>
                            {variant.salePrice ? formatPrice(variant.salePrice) : formatPrice(variant.price)}
                          </span>
                        </div>
                        {!variant.isAvailable && (
                          <span className="text-sm text-muted-foreground">Out of stock</span>
                        )}
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </div>
            )}

                        {/* Colors */}
            {product.colors.length > 0 && (
              <div>
                <h3 className="text-lg font-medium text-foreground mb-4">Available Colors</h3>
                <div className="flex gap-3">
                  {product.colors.map((color, index) => (
                    <div key={index} className="flex flex-col items-center gap-2">
                      <div
                        className="w-10 h-10 rounded-full border-2 border-border shadow-sm"
                        style={{ backgroundColor: getColorValue(color) }}
                        title={color}
                      />
                      <span className="text-xs text-muted-foreground">{color}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Quantity and Add to Cart */}
            <div className="space-y-6">
              <div>
                <label htmlFor="quantity" className="block text-sm font-medium text-foreground mb-2">
                  Quantity
                </label>
                <Select value={quantity.toString()} onValueChange={(value) => setQuantity(Number(value))}>
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
                disabled={product.isSoldOut || (!!selectedVariant && !selectedVariant.isAvailable)}
                size="lg"
                className="w-full"
              >
                {product.isSoldOut || (!!selectedVariant && !selectedVariant.isAvailable)
                  ? 'Out of Stock'
                  : 'Add to Cart'
                }
              </Button>
            </div>

            <Separator />

            {/* Product Details */}
            <div>
              <h3 className="text-lg font-medium text-foreground mb-4">Product Details</h3>
              <div className="space-y-3 text-sm text-muted-foreground">
                {product.material && (
                  <div className="flex justify-between">
                    <span>Material:</span>
                    <span className="text-foreground">{product.material}</span>
                  </div>
                )}
                {product.dimensions && (
                  <div className="flex justify-between">
                    <span>Dimensions:</span>
                    <span className="text-foreground">{product.dimensions}</span>
                  </div>
                )}
                <div className="flex justify-between">
                  <span>Category:</span>
                  <span className="text-foreground capitalize">{product.category.replace('-', ' ')}</span>
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