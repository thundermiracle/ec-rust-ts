import { PaymentMethodListDto } from '$application/dto';
import { PaymentMethod } from '$domain/entities';
import { PaymentMethodId } from '$domain/value-objects';

export interface IPaymentMethodRepository {
  // Query methods - return DTOs
  findAllPaymentMethods(): Promise<PaymentMethodListDto>;

  // Command methods - work with data (no domain entities for simple lookup data)
  findById(id: PaymentMethodId): Promise<PaymentMethod | null>;
}
