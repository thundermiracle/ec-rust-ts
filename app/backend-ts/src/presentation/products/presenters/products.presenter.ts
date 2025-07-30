import { ProductDto, ProductListDto, VariantDto } from '$application/dto';

import {
  ProductDetailResponse,
  ProductListItemResponse,
  ProductListResponse,
  ProductListSimpleResponse,
  ProductResponse,
  VariantResponse,
  VariantSimpleResponse,
} from '../responses';

export class ProductsPresenter {
  static toVariantResponse(dto: VariantDto): VariantResponse {
    return new VariantResponse({
      id: dto.id,
      skuCode: dto.skuCode,
      name: dto.name,
      basePrice: dto.price,
      salePrice: dto.salePrice,
      currentPrice: dto.salePrice || dto.price,
      isOnSale: dto.isOnSale,
      stockQuantity: dto.stockQuantity,
      isInStock: dto.stockQuantity > 0,
      isSoldOut: dto.isSoldOut,
      colorId: null, // Not available in new structure
      colorName: dto.color,
      colorHex: null, // Not available in new structure
      dimensions: dto.dimensions,
      material: dto.material,
      displayOrder: dto.displayOrder,
    });
  }

  static toVariantSimpleResponse(dto: VariantDto): VariantSimpleResponse {
    return new VariantSimpleResponse({
      id: dto.id,
      skuCode: dto.skuCode,
      name: dto.name,
      color: dto.color || 'N/A',
      material: dto.material || 'N/A',
      dimensions: dto.dimensions || 'N/A',
      price: dto.price, // Keep original price without conversion
      salePrice: dto.salePrice || undefined,
      displayOrder: dto.displayOrder,
      image: dto.image || undefined,
      isOnSale: dto.isOnSale,
      isSoldOut: dto.isSoldOut,
    });
  }

  static toProductResponse(dto: ProductDto): ProductDetailResponse {
    const variants = dto.variants.map((variant) =>
      this.toVariantSimpleResponse(variant),
    );

    return new ProductDetailResponse({
      id: dto.id,
      name: dto.name,
      images: dto.images,
      category: dto.category,
      description: dto.description,
      isBestSeller: dto.isBestSeller,
      isQuickShip: dto.isQuickShip,
      variants,
    });
  }

  static toProductListResponse(dto: ProductListDto): ProductListResponse {
    const products = dto.products.map((product) => {
      // For ProductSummaryDto, we don't have variants, so we create an empty array
      const variants: VariantResponse[] = [];

      return new ProductResponse({
        id: product.id,
        name: product.name,
        description: '', // ProductSummaryDto doesn't have description
        categoryId: product.category,
        isBestSeller: product.isBestSeller,
        isQuickShip: product.isQuickShip,
        isAvailable: true, // Assume available if not specified
        variants,
        minPrice: product.basePrice,
        maxPrice: product.basePrice,
        totalStock: product.stockQuantity,
        hasVariants: false, // ProductSummaryDto doesn't track this
      });
    });

    return new ProductListResponse({
      products,
      total: dto.totalCount,
      page: dto.page,
      limit: dto.perPage,
    });
  }

  static toProductListSimpleResponse(
    dto: ProductListDto,
  ): ProductListSimpleResponse {
    const products = dto.products.map((product) => {
      return new ProductListItemResponse({
        id: product.id,
        name: product.name,
        price: product.basePrice, // Keep original price without conversion
        salePrice: product.salePrice || undefined,
        image: product.image || '',
        category: product.category,
        colors: product.colors,
        isOnSale: product.salePrice !== null,
        isBestSeller: product.isBestSeller,
        isQuickShip: product.isQuickShip,
        isSoldOut: product.stockQuantity === 0,
      });
    });

    return new ProductListSimpleResponse({
      products,
      totalCount: dto.totalCount,
      page: dto.page,
      perPage: dto.perPage,
      hasNextPage: dto.hasNextPage,
      hasPreviousPage: dto.hasPreviousPage,
    });
  }
}
