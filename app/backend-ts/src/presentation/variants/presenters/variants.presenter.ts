import {
  VariantSummaryDto,
  FindVariantsItemDto,
} from '../../../application/dto';
import {
  VariantSummaryResponse,
  FindVariantsResponse,
  FindVariantsItemResponse,
} from '../responses';

export class VariantsPresenter {
  static toVariantSummaryResponse(
    dto: VariantSummaryDto,
  ): VariantSummaryResponse {
    return new VariantSummaryResponse({
      id: dto.id,
      skuCode: dto.skuCode,
      productId: dto.productId,
      productName: dto.productName,
      name: dto.name,
      currentPrice: dto.currentPrice,
      isInStock: dto.isInStock,
      stockQuantity: dto.stockQuantity,
      colorId: dto.colorId,
      colorName: dto.colorName,
      colorHex: dto.colorHex,
      dimensions: dto.dimensions,
      material: dto.material,
    });
  }

  static toVariantSummaryListResponse(
    dtos: VariantSummaryDto[],
  ): VariantSummaryResponse[] {
    return dtos.map((dto) => this.toVariantSummaryResponse(dto));
  }

  static toFindVariantsResponse(
    dtos: FindVariantsItemDto[],
  ): FindVariantsResponse {
    const variants = dtos.map(
      (dto) =>
        new FindVariantsItemResponse({
          skuId: dto.skuId,
          price: dto.price,
          salePrice: dto.salePrice || undefined,
          image: dto.image || undefined,
          material: dto.material || undefined,
          dimensions: dto.dimensions || undefined,
        }),
    );

    return new FindVariantsResponse({ variants });
  }
}
