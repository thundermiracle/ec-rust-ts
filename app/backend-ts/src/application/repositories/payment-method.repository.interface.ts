import { PaymentMethodId } from '../../domain/value-objects';
import { PaymentMethodListDto } from '../dto';
import { PaymentMethod } from '../../domain';

export interface IPaymentMethodRepository {
  // Query methods - return DTOs
  findAllPaymentMethods(): Promise<PaymentMethodListDto>;

  // Command methods - work with data (no domain entities for simple lookup data)
  findById(id: PaymentMethodId): Promise<PaymentMethod | null>;
}
