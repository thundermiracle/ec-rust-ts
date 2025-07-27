export class GetProductQuery {
  constructor(public readonly productId: string) {}
}

export class GetProductListQuery {
  constructor(
    public readonly categoryId?: string,
    public readonly page?: number,
    public readonly limit?: number,
  ) {}
}

export class FindVariantsQuery {
  constructor(public readonly skuIds: string[]) {}
}

export class GetCategoryListQuery {
  constructor() {}
}

export class GetColorListQuery {
  constructor() {}
}

export class GetShippingMethodListQuery {
  constructor() {}
}

export class GetPaymentMethodListQuery {
  constructor() {}
}
