import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository, In } from 'typeorm';
import { IProductRepository } from '../../../application/repositories/product.repository.interface';
import { ProductId, SKUId, CategoryId } from '../../../domain/value-objects';
import {
  ProductDto,
  ProductListDto,
  VariantDto,
} from '../../../application/dto';
import { VariantSummaryDto } from '../../../application/dto/variant-summary.dto';
import { ProductEntity } from '../entities/product.entity';
import { SkuEntity } from '../entities/sku.entity';

@Injectable()
export class ProductRepository implements IProductRepository {
  constructor(
    @InjectRepository(ProductEntity)
    private readonly productRepository: Repository<ProductEntity>,
    @InjectRepository(SkuEntity)
    private readonly skuRepository: Repository<SkuEntity>,
  ) {}

  // Query methods - return DTOs
  async findById(id: ProductId): Promise<ProductDto | null> {
    const entity = await this.productRepository.findOne({
      where: { id: id.value() },
      relations: ['skus', 'skus.color'],
    });

    return entity ? this.mapToProductDto(entity) : null;
  }

  async findByCategory(categoryId: CategoryId): Promise<ProductListDto> {
    const entities = await this.productRepository.find({
      where: { category_id: categoryId.value() },
      relations: ['skus', 'skus.color'],
      order: { created_at: 'DESC' },
    });

    const productDtos = entities.map((entity) => this.mapToProductDto(entity));
    return new ProductListDto(
      productDtos,
      productDtos.length,
      1,
      productDtos.length,
    );
  }

  async findAll(): Promise<ProductListDto> {
    const entities = await this.productRepository.find({
      relations: ['skus', 'skus.color'],
      order: { created_at: 'DESC' },
    });

    const productDtos = entities.map((entity) => this.mapToProductDto(entity));
    return new ProductListDto(
      productDtos,
      productDtos.length,
      1,
      productDtos.length,
    );
  }

  async findSkusByIds(skuIds: SKUId[]): Promise<VariantSummaryDto[]> {
    const entities = await this.skuRepository.find({
      where: { id: In(skuIds.map((id) => id.value())) },
      relations: ['product', 'color'],
    });

    return entities.map((entity) => this.mapToVariantSummaryDto(entity));
  }

  private mapToProductDto(entity: ProductEntity): ProductDto {
    const variants = entity.skus?.map((sku) => this.mapToVariantDto(sku)) || [];

    // Calculate price range
    const prices = variants
      .map((v) => v.currentPrice)
      .filter((p) => p !== null);
    const minPrice = prices.length > 0 ? Math.min(...prices) : null;
    const maxPrice = prices.length > 0 ? Math.max(...prices) : null;

    // Calculate total stock
    const totalStock = variants.reduce((sum, v) => sum + v.stockQuantity, 0);

    return new ProductDto(
      entity.id,
      entity.name,
      entity.description,
      entity.category_id,
      entity.is_best_seller,
      entity.is_quick_ship,
      entity.is_available,
      variants,
      minPrice,
      maxPrice,
      totalStock,
      variants.length > 1,
    );
  }

  private mapToVariantDto(skuEntity: SkuEntity): VariantDto {
    return new VariantDto(
      skuEntity.id,
      skuEntity.sku_code,
      skuEntity.name,
      skuEntity.base_price,
      skuEntity.sale_price || null,
      skuEntity.sale_price || skuEntity.base_price,
      skuEntity.sale_price !== null,
      skuEntity.stock_quantity,
      skuEntity.stock_quantity > 0,
      skuEntity.stock_quantity === 0,
      skuEntity.color_id || null,
      skuEntity.color?.name || null,
      skuEntity.color?.hex || null,
      skuEntity.dimensions || null,
      skuEntity.material || null,
      skuEntity.display_order,
    );
  }

  private mapToVariantSummaryDto(skuEntity: SkuEntity): VariantSummaryDto {
    return new VariantSummaryDto(
      skuEntity.id,
      skuEntity.sku_code,
      skuEntity.product_id,
      skuEntity.product?.name || '',
      skuEntity.name,
      skuEntity.sale_price || skuEntity.base_price,
      skuEntity.stock_quantity > 0,
      skuEntity.stock_quantity,
      skuEntity.color_id || null,
      skuEntity.color?.name || null,
      skuEntity.color?.hex || null,
      skuEntity.dimensions || null,
      skuEntity.material || null,
    );
  }
}
