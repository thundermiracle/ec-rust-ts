'use client';

import Link from 'next/link';
import { useState, useEffect, FC } from 'react';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { ShoppingBag, Menu, X } from 'lucide-react';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { selectCartItemsCount, toggleCart, initializeCart } from '@/store/cartSlice';
import { CartDrawer } from '@/components/Cart';

const Header: FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const dispatch = useAppDispatch();
  const cartItemsCount = useAppSelector(selectCartItemsCount);

  // Initialize cart from localStorage on mount
  useEffect(() => {
    dispatch(initializeCart());
  }, [dispatch]);

  const handleCartClick = () => {
    dispatch(toggleCart());
  };

  return (
    <>
      <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex h-16 items-center justify-between">
            {/* Logo */}
            <div className="flex items-center">
              <Link href="/" className="text-2xl font-bold tracking-[0.2em] text-foreground hover:text-foreground/80 transition-colors">
                A R T I F O X
              </Link>
            </div>

            {/* Desktop Navigation */}
            <nav className="hidden md:flex items-center space-x-8">
              <Link href="/collections" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Shop All
              </Link>
              <Button variant="ghost" size="sm" className="text-sm text-muted-foreground hover:text-foreground">
                Collections
              </Button>
              <Button variant="ghost" size="sm" className="text-sm text-muted-foreground hover:text-foreground">
                Featured
              </Button>
              <Button variant="ghost" size="sm" className="text-sm text-muted-foreground hover:text-foreground">
                Color
              </Button>
              <Link href="/about" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                About
              </Link>
              <Link href="/blog" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Blog
              </Link>
              <Link href="/contact" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Contact
              </Link>
            </nav>

            {/* Cart Icon */}
            <div className="flex items-center space-x-4">
              <Button variant="ghost" size="sm" className="relative cursor-pointer" onClick={handleCartClick}>
                <ShoppingBag className="h-5 w-5" />
                {cartItemsCount > 0 && (
                  <span className="absolute -top-1 -right-1 bg-primary text-primary-foreground text-xs rounded-full h-5 w-5 flex items-center justify-center min-w-[20px]">
                    {cartItemsCount > 99 ? '99+' : cartItemsCount}
                  </span>
                )}
                <span className="sr-only">Shopping cart ({cartItemsCount})</span>
              </Button>
              
              {/* Mobile menu button */}
              <Button
                variant="ghost"
                size="sm"
                className="md:hidden"
                onClick={() => setIsMenuOpen(!isMenuOpen)}
              >
                {isMenuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
                <span className="sr-only">Toggle menu</span>
              </Button>
            </div>
          </div>

          {/* Mobile Navigation */}
          {isMenuOpen && (
            <>
              <Separator />
              <div className="md:hidden py-4">
                <nav className="flex flex-col space-y-4">
                  <Link href="/collections" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    Shop All
                  </Link>
                  <Link href="/collections" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    Collections
                  </Link>
                  <Link href="/featured" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    Featured
                  </Link>
                  <Link href="/about" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    About
                  </Link>
                  <Link href="/blog" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    Blog
                  </Link>
                  <Link href="/contact" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    Contact
                  </Link>
                </nav>
              </div>
            </>
          )}
        </div>
      </header>

      {/* Cart Drawer */}
      <CartDrawer />
    </>
  );
};

export { Header }; 