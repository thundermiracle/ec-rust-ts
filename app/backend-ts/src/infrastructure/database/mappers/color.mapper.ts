import { Color } from '../../../domain/entities';
import { ColorId } from '../../../domain/value-objects';
import { ColorEntity } from '../entities/color.entity';

export class ColorMapper {
  static toDomain(entity: ColorEntity): Color {
    return Color.create(ColorId.new(entity.id), entity.name, entity.hex);
  }

  static toEntity(domain: Color): ColorEntity {
    const entity = new ColorEntity();
    entity.id = domain.getId().value();
    entity.name = domain.getName().getValue();
    entity.hex = domain.getHex();
    return entity;
  }
}
