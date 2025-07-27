import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { ICategoryRepository } from '../../../application/repositories/category.repository.interface';
import { Category } from '../../../domain/entities';
import { CategoryId } from '../../../domain/value-objects';
import { CategoryDto, CategoryListDto } from '../../../application/dto';
import { CategoryEntity } from '../entities/category.entity';
import { CategoryMapper } from '../mappers/category.mapper';

@Injectable()
export class CategoryRepository implements ICategoryRepository {
  constructor(
    @InjectRepository(CategoryEntity)
    private readonly categoryRepository: Repository<CategoryEntity>,
  ) {}

  // Query methods - return DTOs
  async findAllCategories(): Promise<CategoryListDto> {
    const entities = await this.categoryRepository.find({
      order: { display_order: 'ASC', name: 'ASC' },
    });

    const categoryDtos = entities.map(
      (entity) =>
        new CategoryDto(
          entity.id,
          entity.name,
          entity.slug,
          entity.parent_id || null,
          entity.display_order,
        ),
    );

    return new CategoryListDto(categoryDtos);
  }

  // Command methods - work with entities
  async findById(id: CategoryId): Promise<Category | null> {
    const entity = await this.categoryRepository.findOne({
      where: { id: id.value() },
    });

    return entity ? CategoryMapper.toDomain(entity) : null;
  }
}
