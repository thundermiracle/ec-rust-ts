import { CategoryId, ProductId } from './identifiers';
import { Money } from './money';

export abstract class DiscountCondition {
  abstract getType(): string;
}

export class MinimumPurchaseCondition extends DiscountCondition {
  constructor(private readonly minimumAmount: Money) {
    super();
  }

  getMinimumAmount(): Money {
    return this.minimumAmount;
  }

  getType(): string {
    return 'MinimumPurchase';
  }

  equals(other: MinimumPurchaseCondition): boolean {
    return this.minimumAmount.equals(other.minimumAmount);
  }
}

export class ProductSpecificCondition extends DiscountCondition {
  constructor(private readonly productIds: ProductId[]) {
    super();
    if (productIds.length === 0) {
      throw new Error(
        'Product specific condition requires at least one product ID',
      );
    }
  }

  getProductIds(): ProductId[] {
    return [...this.productIds];
  }

  getType(): string {
    return 'ProductSpecific';
  }

  equals(other: ProductSpecificCondition): boolean {
    const thisIds = this.productIds.map((id) => id.value()).sort();
    const otherIds = other.productIds.map((id) => id.value()).sort();
    return JSON.stringify(thisIds) === JSON.stringify(otherIds);
  }
}

export class CategorySpecificCondition extends DiscountCondition {
  constructor(private readonly categoryIds: CategoryId[]) {
    super();
    if (categoryIds.length === 0) {
      throw new Error(
        'Category specific condition requires at least one category ID',
      );
    }
  }

  getCategoryIds(): CategoryId[] {
    return [...this.categoryIds];
  }

  getType(): string {
    return 'CategorySpecific';
  }

  equals(other: CategorySpecificCondition): boolean {
    const thisIds = this.categoryIds.map((id) => id.value()).sort();
    const otherIds = other.categoryIds.map((id) => id.value()).sort();
    return JSON.stringify(thisIds) === JSON.stringify(otherIds);
  }
}
