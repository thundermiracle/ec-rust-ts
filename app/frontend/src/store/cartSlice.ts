import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface CartItem {
  id: string;
  name: string;
  price: number;
  quantity: number;
  image: string;
  skuId?: string;
  color?: string;
  size?: string;
}

export interface CartState {
  items: CartItem[];
  isOpen: boolean;
}

const initialState: CartState = {
  items: [],
  isOpen: false,
};

// LocalStorage utility functions
const CART_STORAGE_KEY = 'artifox_cart';

const loadCartFromStorage = (): CartItem[] => {
  if (typeof window === 'undefined') return [];
  
  try {
    const stored = localStorage.getItem(CART_STORAGE_KEY);
    return stored ? JSON.parse(stored) : [];
  } catch (error) {
    console.error('Error loading cart from localStorage:', error);
    return [];
  }
};

const saveCartToStorage = (items: CartItem[]) => {
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
    },
    
    addToCart: (state, action: PayloadAction<Omit<CartItem, 'quantity'> & { quantity?: number }>) => {
      const { quantity = 1, ...item } = action.payload;
      const existingItem = state.items.find(
        (cartItem) => cartItem.id === item.id && cartItem.skuId === item.skuId
      );

      if (existingItem) {
        existingItem.quantity += quantity;
      } else {
        state.items.push({ ...item, quantity });
      }

      saveCartToStorage(state.items);
    },

    removeFromCart: (state, action: PayloadAction<{ id: string; skuId?: string }>) => {
      const { id, skuId } = action.payload;
      state.items = state.items.filter(
        (item) => !(item.id === id && item.skuId === skuId)
      );
      saveCartToStorage(state.items);
    },

    updateQuantity: (state, action: PayloadAction<{ id: string; skuId?: string; quantity: number }>) => {
      const { id, skuId, quantity } = action.payload;
      const item = state.items.find(
        (cartItem) => cartItem.id === id && cartItem.skuId === skuId
      );

      if (item) {
        if (quantity <= 0) {
          state.items = state.items.filter(
            (cartItem) => !(cartItem.id === id && cartItem.skuId === skuId)
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
export const selectCartItems = (state: { cart: CartState }) => state.cart.items;
export const selectCartIsOpen = (state: { cart: CartState }) => state.cart.isOpen;
export const selectCartItemsCount = (state: { cart: CartState }) => 
  state.cart.items.reduce((total, item) => total + item.quantity, 0);
export const selectCartTotal = (state: { cart: CartState }) => 
  state.cart.items.reduce((total, item) => total + (item.price * item.quantity), 0);

export default cartSlice.reducer; 