export class CategoryDto {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly slug: string,
    public readonly parentId: string | null,
    public readonly displayOrder: number,
  ) {}
}

export class CategoryListDto {
  constructor(public readonly categories: CategoryDto[]) {}
}
