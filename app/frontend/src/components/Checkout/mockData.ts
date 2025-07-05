export const SHIPPING_OPTIONS = [
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

export const PAYMENT_OPTIONS = [
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

export type ShippingMethodType = typeof SHIPPING_OPTIONS[number]['id'];
export type PaymentMethodType = typeof PAYMENT_OPTIONS[number]['id']; 