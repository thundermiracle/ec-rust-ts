import { CategoryListDto } from '$application/dto';
import { Category } from '$domain/entities';
import { CategoryId } from '$domain/value-objects';

export interface ICategoryRepository {
  // Query methods - return DTOs
  findAllCategories(): Promise<CategoryListDto>;

  // Command methods - work with entities
  findById(id: CategoryId): Promise<Category | null>; // For business logic
}
