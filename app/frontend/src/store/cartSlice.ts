import { createSlice, PayloadAction } from '@reduxjs/toolkit';

// LocalStorageに保存する軽量なカートアイテム
export interface StoredCartItem {
  productId: string;
  skuId: string;  // 必須プロパティ
  quantity: number;
}

export interface CartState {
  items: StoredCartItem[];
  isOpen: boolean;
  initialized: boolean;
}

const initialState: CartState = {
  items: [],
  isOpen: false,
  initialized: false,
};

// LocalStorage utility functions
const CART_STORAGE_KEY = 'artifox_cart';

const loadCartFromStorage = (): StoredCartItem[] => {
  if (typeof window === 'undefined') return [];
  
  try {
    const stored = localStorage.getItem(CART_STORAGE_KEY);
    return stored ? JSON.parse(stored) : [];
  } catch (error) {
    console.error('Error loading cart from localStorage:', error);
    return [];
  }
};

const saveCartToStorage = (items: StoredCartItem[]) => {
  if (typeof window === 'undefined') return;
  
  try {
    localStorage.setItem(CART_STORAGE_KEY, JSON.stringify(items));
  } catch (error) {
    console.error('Error saving cart to localStorage:', error);
  }
};

const cartSlice = createSlice({
  name: 'cart',
  initialState,
  reducers: {
    initializeCart: (state) => {
      state.items = loadCartFromStorage();
      state.initialized = true;
    },
    
    addToCart: (state, action: PayloadAction<{ productId: string; skuId: string; quantity?: number }>) => {
      const { productId, skuId, quantity = 1 } = action.payload;
      const existingItem = state.items.find(
        (cartItem) => cartItem.productId === productId && cartItem.skuId === skuId
      );

      if (existingItem) {
        existingItem.quantity += quantity;
      } else {
        state.items.push({ productId, skuId, quantity });
      }

      saveCartToStorage(state.items);
    },

    removeFromCart: (state, action: PayloadAction<{ productId: string; skuId: string }>) => {
      const { productId, skuId } = action.payload;
      state.items = state.items.filter(
        (item) => !(item.productId === productId && item.skuId === skuId)
      );
      saveCartToStorage(state.items);
    },

    updateQuantity: (state, action: PayloadAction<{ productId: string; skuId: string; quantity: number }>) => {
      const { productId, skuId, quantity } = action.payload;
      const item = state.items.find(
        (cartItem) => cartItem.productId === productId && cartItem.skuId === skuId
      );

      if (item) {
        if (quantity <= 0) {
          state.items = state.items.filter(
            (cartItem) => !(cartItem.productId === productId && cartItem.skuId === skuId)
          );
        } else {
          item.quantity = quantity;
        }
        saveCartToStorage(state.items);
      }
    },

    clearCart: (state) => {
      state.items = [];
      saveCartToStorage(state.items);
    },

    setCartOpen: (state, action: PayloadAction<boolean>) => {
      state.isOpen = action.payload;
    },

    toggleCart: (state) => {
      state.isOpen = !state.isOpen;
    },
  },
});

export const {
  initializeCart,
  addToCart,
  removeFromCart,
  updateQuantity,
  clearCart,
  setCartOpen,
  toggleCart,
} = cartSlice.actions;

// Selectors
export const selectCartItems = (state: { cart: CartState }) => {
  // Lazily load from localStorage if cart has not been initialized yet
  if (!state.cart.initialized) {
    return loadCartFromStorage();
  }
  return state.cart.items;
};

export const selectCartInitialized = (state: { cart: CartState }) => state.cart.initialized;

export const selectCartIsOpen = (state: { cart: CartState }) => state.cart.isOpen;

export const selectCartItemsCount = (state: { cart: CartState }) => 
  selectCartItems(state).reduce((total, item) => total + item.quantity, 0);

export default cartSlice.reducer; 