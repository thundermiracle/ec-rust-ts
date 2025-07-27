import { ShippingMethodId } from '../../domain/value-objects';
import { Money } from '../../domain/value-objects';
import { ShippingMethodListDto } from '../dto';

export interface ShippingMethodData {
  id: ShippingMethodId;
  name: string;
  fee: Money;
  description?: string;
}

export interface IShippingMethodRepository {
  // Query methods - return DTOs
  findAllShippingMethods(): Promise<ShippingMethodListDto>;

  // Command methods - work with data (no domain entities for simple lookup data)
  findById(id: ShippingMethodId): Promise<ShippingMethodData | null>;
}
