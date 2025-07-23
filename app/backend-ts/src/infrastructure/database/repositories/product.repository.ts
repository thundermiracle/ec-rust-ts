import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, In } from 'typeorm';
import { IProductRepository } from '../../../application/repositories/product.repository.interface';
import { Product, SKU } from '../../../domain/entities';
import { ProductId, SKUId, CategoryId } from '../../../domain/value-objects';
import { ProductEntity } from '../entities/product.entity';
import { SkuEntity } from '../entities/sku.entity';
import { ProductMapper, SkuMapper } from '../mappers/product.mapper';

@Injectable()
export class ProductRepository implements IProductRepository {
  constructor(
    @InjectRepository(ProductEntity)
    private readonly productRepository: Repository<ProductEntity>,
    @InjectRepository(SkuEntity)
    private readonly skuRepository: Repository<SkuEntity>,
  ) {}

  async findById(id: ProductId): Promise<Product | null> {
    const entity = await this.productRepository.findOne({
      where: { id: id.value() },
      relations: ['skus', 'category', 'images'],
    });

    return entity ? ProductMapper.toDomain(entity) : null;
  }

  async findByCategory(categoryId: CategoryId): Promise<Product[]> {
    const entities = await this.productRepository.find({
      where: { category_id: categoryId.value() },
      relations: ['skus', 'category', 'images'],
    });

    return entities.map((entity) => ProductMapper.toDomain(entity));
  }

  async findAll(): Promise<Product[]> {
    const entities = await this.productRepository.find({
      relations: ['skus', 'category', 'images'],
    });

    return entities.map((entity) => ProductMapper.toDomain(entity));
  }

  async findSkuById(skuId: SKUId): Promise<SKU | null> {
    const entity = await this.skuRepository.findOne({
      where: { id: skuId.value() },
      relations: ['product', 'color'],
    });

    return entity ? SkuMapper.toDomain(entity) : null;
  }

  async findSkusByIds(skuIds: SKUId[]): Promise<SKU[]> {
    const ids = skuIds.map((id) => id.value());
    const entities = await this.skuRepository.find({
      where: { id: In(ids) },
      relations: ['product', 'color'],
    });

    return entities.map((entity) => SkuMapper.toDomain(entity));
  }

  async save(product: Product): Promise<void> {
    const entity = ProductMapper.toEntity(product);
    await this.productRepository.save(entity);

    // Save SKUs separately
    for (const sku of product.getSkus()) {
      const skuEntity = SkuMapper.toEntity(sku);
      await this.skuRepository.save(skuEntity);
    }
  }

  async update(product: Product): Promise<void> {
    const entity = ProductMapper.toEntity(product);
    await this.productRepository.save(entity);

    // Update SKUs
    for (const sku of product.getSkus()) {
      const skuEntity = SkuMapper.toEntity(sku);
      await this.skuRepository.save(skuEntity);
    }
  }

  async delete(id: ProductId): Promise<void> {
    await this.productRepository.delete(id.value());
  }
}
