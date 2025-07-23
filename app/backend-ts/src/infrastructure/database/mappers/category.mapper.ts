import { Category } from '../../../domain/entities';
import { CategoryId } from '../../../domain/value-objects';
import { CategoryEntity } from '../entities/category.entity';

export class CategoryMapper {
  static toDomain(entity: CategoryEntity): Category {
    return Category.create(
      CategoryId.fromUuid(entity.id),
      entity.name,
      entity.slug,
      entity.parent_id ? CategoryId.fromUuid(entity.parent_id) : undefined,
      entity.display_order,
    );
  }

  static toEntity(domain: Category): CategoryEntity {
    const entity = new CategoryEntity();
    entity.id = domain.getId().value();
    entity.name = domain.getName();
    entity.slug = domain.getSlug();
    entity.parent_id = domain.getParentId()?.value();
    entity.display_order = domain.getDisplayOrder();
    return entity;
  }
}
