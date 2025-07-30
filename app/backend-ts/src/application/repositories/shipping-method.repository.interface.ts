import { ShippingMethodListDto } from '$application/dto';
import { ShippingMethod } from '$domain/entities';
import { ShippingMethodId } from '$domain/value-objects';

export interface IShippingMethodRepository {
  // Query methods - return DTOs
  findAllShippingMethods(): Promise<ShippingMethodListDto>;

  // Command methods - work with data (no domain entities for simple lookup data)
  findById(id: ShippingMethodId): Promise<ShippingMethod | null>;
}
