// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { IPresenter } from '../../base';
import { CategoryListDto, CategoryDto } from '../../../application/dto';
import { CategoryListResponse, CategoryResponse } from '../responses';

export class CategoriesPresenter {
  static toCategoryResponse(dto: CategoryDto): CategoryResponse {
    return new CategoryResponse({
      id: dto.id,
      name: dto.name,
      slug: dto.slug,
      parentId: dto.parentId,
      displayOrder: dto.displayOrder,
    });
  }

  static toCategoryListResponse(dto: CategoryListDto): CategoryListResponse {
    const categories = dto.categories.map((category) =>
      this.toCategoryResponse(category),
    );

    return new CategoryListResponse({ categories });
  }
}
