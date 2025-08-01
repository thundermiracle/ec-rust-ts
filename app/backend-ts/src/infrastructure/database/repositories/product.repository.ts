import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { In, Repository } from 'typeorm';

import {
  ProductDto,
  ProductListDto,
  ProductSummaryDto,
  VariantDto,
} from '$application/dto';
import { FindVariantsItemDto } from '$application/dto/find-variants.dto';
import { VariantSummaryDto } from '$application/dto/variant-summary.dto';
import { IProductRepository } from '$application/repositories/product.repository.interface';
import { SKU, VariantAttributes } from '$domain/entities/sku';
import {
  CategoryId,
  ColorId,
  Money,
  ProductId,
  SKUId,
} from '$domain/value-objects';

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
      relations: ['skus', 'skus.color', 'images', 'category'],
    });

    return entity ? this.mapToProductDto(entity) : null;
  }

  async findByCategory(categoryId: CategoryId): Promise<ProductListDto> {
    const entities = await this.productRepository.find({
      where: { category_id: categoryId.value() },
      relations: ['skus', 'skus.color', 'images', 'category'],
      order: { created_at: 'DESC' },
    });

    const productSummaries = entities.map((entity) =>
      this.mapToProductSummaryDto(entity),
    );
    const totalCount = productSummaries.length;
    const page = 1;
    const perPage = productSummaries.length;

    return new ProductListDto(
      productSummaries,
      totalCount,
      page,
      perPage,
      false, // hasNextPage
      false, // hasPreviousPage
    );
  }

  async findAll(): Promise<ProductListDto> {
    const entities = await this.productRepository.find({
      relations: ['skus', 'skus.color', 'images', 'category'],
      order: { created_at: 'DESC' },
    });

    const productSummaries = entities.map((entity) =>
      this.mapToProductSummaryDto(entity),
    );
    const totalCount = productSummaries.length;
    const page = 1;
    const perPage = productSummaries.length;

    return new ProductListDto(
      productSummaries,
      totalCount,
      page,
      perPage,
      false, // hasNextPage
      false, // hasPreviousPage
    );
  }

  async findSkusByIds(skuIds: SKUId[]): Promise<VariantSummaryDto[]> {
    const entities = await this.skuRepository.find({
      where: { id: In(skuIds.map((id) => id.value())) },
      relations: ['product', 'color'],
    });

    return entities.map((entity) => this.mapToVariantSummaryDto(entity));
  }

  async findVariantsBySkuIds(skuIds: SKUId[]): Promise<FindVariantsItemDto[]> {
    const entities = await this.skuRepository.find({
      where: { id: In(skuIds.map((id) => id.value())) },
      relations: ['product'],
    });

    return entities.map((entity) => this.mapToFindVariantsItemDto(entity));
  }

  async findSkuEntitiesByIds(skuIds: SKUId[]): Promise<SKU[]> {
    const entities = await this.skuRepository.find({
      where: { id: In(skuIds.map((id) => id.value())) },
      relations: ['product', 'color'],
    });

    return entities.map((entity) => this.mapToSkuEntity(entity));
  }

  private mapToProductDto(entity: ProductEntity): ProductDto {
    const variants = entity.skus?.map((sku) => this.mapToVariantDto(sku)) || [];

    // Get actual images from database, sorted by display_order
    const images =
      entity.images
        ?.sort((a, b) => a.display_order - b.display_order)
        .map((img) => img.image_url) || [];

    return new ProductDto(
      entity.id,
      entity.name,
      images,
      entity.category?.name || '', // Use category name instead of ID
      entity.description,
      entity.is_best_seller,
      entity.is_quick_ship,
      variants,
    );
  }

  private mapToVariantDto(skuEntity: SkuEntity): VariantDto {
    const isOnSale = skuEntity.sale_price !== null;
    const isSoldOut = skuEntity.stock_quantity === 0;

    return new VariantDto(
      skuEntity.id,
      skuEntity.sku_code,
      skuEntity.name,
      skuEntity.color?.name || '',
      skuEntity.material || '',
      skuEntity.dimensions || '',
      skuEntity.base_price,
      skuEntity.sale_price || null,
      skuEntity.stock_quantity,
      skuEntity.reserved_quantity,
      skuEntity.display_order,
      skuEntity.image_url || null,
      isOnSale,
      isSoldOut,
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

  private mapToProductSummaryDto(entity: ProductEntity): ProductSummaryDto {
    const variants = entity.skus || [];

    // Calculate base price from variants
    const basePrice =
      variants.length > 0 ? Math.min(...variants.map((v) => v.base_price)) : 0;

    // Calculate sale price if any variant has sale price
    const salePrice =
      variants.find((v) => v.sale_price !== null)?.sale_price || null;

    // Get all colors
    const colors = variants
      .map((v) => v.color?.name)
      .filter(
        (color): color is string => color !== null && color !== undefined,
      );

    // Calculate total stock
    const stockQuantity = variants.reduce(
      (sum, v) => sum + v.stock_quantity,
      0,
    );

    // Get first image from database, sorted by display_order
    const image =
      entity.images?.sort((a, b) => a.display_order - b.display_order)[0]
        ?.image_url || null;

    return new ProductSummaryDto(
      entity.id,
      entity.name,
      entity.category?.name || '', // Use category name instead of ID
      basePrice,
      salePrice,
      image,
      colors,
      entity.is_best_seller,
      entity.is_quick_ship,
      stockQuantity,
    );
  }

  private mapToFindVariantsItemDto(skuEntity: SkuEntity): FindVariantsItemDto {
    return new FindVariantsItemDto(
      skuEntity.id,
      skuEntity.base_price, // Keep original price without conversion
      skuEntity.sale_price || null,
      skuEntity.image_url || null,
      skuEntity.material || null,
      skuEntity.dimensions || null,
    );
  }

  private mapToSkuEntity(entity: SkuEntity): SKU {
    // Create variant attributes
    const variantAttributes: VariantAttributes = {
      colorId: entity.color_id ? ColorId.new(entity.color_id) : undefined,
      dimensions: entity.dimensions || undefined,
      material: entity.material || undefined,
    };

    // Create SKU entity
    const sku = SKU.create(
      SKUId.fromUuid(entity.id),
      ProductId.fromUuid(entity.product_id),
      entity.sku_code,
      entity.name,
      variantAttributes,
      Money.fromYen(entity.base_price),
      entity.stock_quantity,
      entity.display_order,
    );

    // Set sale price if exists
    if (entity.sale_price) {
      sku.setSalePrice(Money.fromYen(entity.sale_price));
    }

    // Note: Status handling can be added when status field is available in SkuEntity

    return sku;
  }
}
