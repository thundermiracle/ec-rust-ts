'use client';

import { Card, CardContent } from '@/components/ui/card';
import { Loader2 } from 'lucide-react';
import { FC } from 'react';

export const ProductLoadingState: FC = () => {  
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