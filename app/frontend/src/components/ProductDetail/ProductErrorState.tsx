'use client';

import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { FC } from 'react';

interface ProductErrorStateProps {
  isNotFound?: boolean;
}

export const ProductErrorState: FC<ProductErrorStateProps> = ({ isNotFound = false }) => {
  if (isNotFound) {
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