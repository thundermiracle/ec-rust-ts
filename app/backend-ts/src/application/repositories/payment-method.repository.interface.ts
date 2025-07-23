import { PaymentMethodId } from '../../domain/value-objects';
import { Money } from '../../domain/value-objects';

export interface PaymentMethodData {
  id: PaymentMethodId;
  name: string;
  fee: Money;
  description?: string;
}

export interface IPaymentMethodRepository {
  findById(id: PaymentMethodId): Promise<PaymentMethodData | null>;
  findAll(): Promise<PaymentMethodData[]>;
  save(method: PaymentMethodData): Promise<void>;
  update(method: PaymentMethodData): Promise<void>;
  delete(id: PaymentMethodId): Promise<void>;
}
