import { Color } from '../../domain/entities';
import { ColorId } from '../../domain/value-objects';

export interface IColorRepository {
  findById(id: ColorId): Promise<Color | null>;
  findAll(): Promise<Color[]>;
  save(color: Color): Promise<void>;
  update(color: Color): Promise<void>;
  delete(id: ColorId): Promise<void>;
}
