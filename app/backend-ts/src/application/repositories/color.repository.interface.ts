import { ColorListDto } from '../dto';

export interface IColorRepository {
  // Query methods - return DTOs
  findAllColors(): Promise<ColorListDto>;
}
