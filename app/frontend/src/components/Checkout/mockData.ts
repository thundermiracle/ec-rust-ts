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

export type PaymentMethodType = typeof PAYMENT_OPTIONS[number]['id']; 