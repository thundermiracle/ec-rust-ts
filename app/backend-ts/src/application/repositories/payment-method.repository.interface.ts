import { PaymentMethodId } from '../../domain/value-objects';
import { Money } from '../../domain/value-objects';
import { PaymentMethodListDto } from '../dto';

export interface PaymentMethodData {
  id: PaymentMethodId;
  name: string;
  fee: Money;
  description?: string;
}

export interface IPaymentMethodRepository {
  // Query methods - return DTOs
  findAllPaymentMethods(): Promise<PaymentMethodListDto>;

  // Command methods - work with data (no domain entities for simple lookup data)
  findById(id: PaymentMethodId): Promise<PaymentMethodData | null>;
}
