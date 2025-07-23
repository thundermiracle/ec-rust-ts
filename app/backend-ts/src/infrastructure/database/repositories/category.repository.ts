import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, IsNull } from 'typeorm';
import { ICategoryRepository } from '../../../application/repositories/category.repository.interface';
import { Category } from '../../../domain/entities';
import { CategoryId } from '../../../domain/value-objects';
import { CategoryEntity } from '../entities/category.entity';
import { CategoryMapper } from '../mappers/category.mapper';

@Injectable()
export class CategoryRepository implements ICategoryRepository {
  constructor(
    @InjectRepository(CategoryEntity)
    private readonly categoryRepository: Repository<CategoryEntity>,
  ) {}

  async findById(id: CategoryId): Promise<Category | null> {
    const entity = await this.categoryRepository.findOne({
      where: { id: id.value() },
    });

    return entity ? CategoryMapper.toDomain(entity) : null;
  }

  async findAll(): Promise<Category[]> {
    const entities = await this.categoryRepository.find({
      order: { display_order: 'ASC', name: 'ASC' },
    });

    return entities.map((entity) => CategoryMapper.toDomain(entity));
  }

  async findRootCategories(): Promise<Category[]> {
    const entities = await this.categoryRepository.find({
      where: { parent_id: IsNull() },
      order: { display_order: 'ASC', name: 'ASC' },
    });

    return entities.map((entity) => CategoryMapper.toDomain(entity));
  }

  async findByParentId(parentId: CategoryId): Promise<Category[]> {
    const entities = await this.categoryRepository.find({
      where: { parent_id: parentId.value() },
      order: { display_order: 'ASC', name: 'ASC' },
    });

    return entities.map((entity) => CategoryMapper.toDomain(entity));
  }

  async save(category: Category): Promise<void> {
    const entity = CategoryMapper.toEntity(category);
    await this.categoryRepository.save(entity);
  }

  async update(category: Category): Promise<void> {
    const entity = CategoryMapper.toEntity(category);
    await this.categoryRepository.save(entity);
  }

  async delete(id: CategoryId): Promise<void> {
    await this.categoryRepository.delete(id.value());
  }
}
