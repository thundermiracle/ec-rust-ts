import { ColorListDto } from '$application/dto';

export interface IColorRepository {
  // Query methods - return DTOs
  findAllColors(): Promise<ColorListDto>;
}
