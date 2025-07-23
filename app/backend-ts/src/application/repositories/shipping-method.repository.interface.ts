import { ShippingMethodId } from '../../domain/value-objects';
import { Money } from '../../domain/value-objects';

export interface ShippingMethodData {
  id: ShippingMethodId;
  name: string;
  fee: Money;
  description?: string;
}

export interface IShippingMethodRepository {
  findById(id: ShippingMethodId): Promise<ShippingMethodData | null>;
  findAll(): Promise<ShippingMethodData[]>;
  save(method: ShippingMethodData): Promise<void>;
  update(method: ShippingMethodData): Promise<void>;
  delete(id: ShippingMethodId): Promise<void>;
}
