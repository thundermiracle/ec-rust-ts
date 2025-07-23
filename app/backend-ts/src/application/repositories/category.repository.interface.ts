import { Category } from '../../domain/entities';
import { CategoryId } from '../../domain/value-objects';

export interface ICategoryRepository {
  findById(id: CategoryId): Promise<Category | null>;
  findAll(): Promise<Category[]>;
  findRootCategories(): Promise<Category[]>;
  findByParentId(parentId: CategoryId): Promise<Category[]>;
  save(category: Category): Promise<void>;
  update(category: Category): Promise<void>;
  delete(id: CategoryId): Promise<void>;
}
