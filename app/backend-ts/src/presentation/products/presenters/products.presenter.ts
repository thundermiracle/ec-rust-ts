import {
  ProductListDto,
  ProductDto,
  VariantDto,
} from '../../../application/dto';
import {
  ProductListResponse,
  ProductDetailResponse,
  VariantResponse,
} from '../responses';

export class ProductsPresenter {
  static toVariantResponse(dto: VariantDto): VariantResponse {
    return new VariantResponse({
      id: dto.id,
      skuCode: dto.skuCode,
      name: dto.name,
      basePrice: dto.basePrice,
      salePrice: dto.salePrice,
      currentPrice: dto.currentPrice,
      isOnSale: dto.isOnSale,
      stockQuantity: dto.stockQuantity,
      isInStock: dto.isInStock,
      isSoldOut: dto.isSoldOut,
      colorId: dto.colorId,
      colorName: dto.colorName,
      colorHex: dto.colorHex,
      dimensions: dto.dimensions,
      material: dto.material,
      displayOrder: dto.displayOrder,
    });
  }

  static toProductResponse(dto: ProductDto): ProductDetailResponse {
    const variants = dto.variants.map((variant) =>
      this.toVariantResponse(variant),
    );

    return new ProductDetailResponse({
      id: dto.id,
      name: dto.name,
      description: dto.description,
      categoryId: dto.categoryId,
      isBestSeller: dto.isBestSeller,
      isQuickShip: dto.isQuickShip,
      isAvailable: dto.isAvailable,
      variants,
      minPrice: dto.minPrice,
      maxPrice: dto.maxPrice,
      totalStock: dto.totalStock,
      hasVariants: dto.hasVariants,
    });
  }

  static toProductListResponse(dto: ProductListDto): ProductListResponse {
    const products = dto.products.map((product) =>
      this.toProductResponse(product),
    );

    return new ProductListResponse({
      products,
      total: dto.total,
      page: dto.page,
      limit: dto.limit,
    });
  }
}
