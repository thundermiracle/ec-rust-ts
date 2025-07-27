import { VariantSummaryDto } from '../../../application/dto';
import { VariantSummaryResponse } from '../responses';

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
}
