// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { IPresenter } from '../../base';
import { ColorListDto, ColorDto } from '../../../application/dto';
import { ColorListResponse, ColorResponse } from '../responses';

export class ColorsPresenter {
  static toColorResponse(dto: ColorDto): ColorResponse {
    return new ColorResponse({
      id: dto.id,
      name: dto.name,
      hex: dto.hex,
    });
  }

  static toColorListResponse(dto: ColorListDto): ColorListResponse {
    const colors = dto.colors.map((color) => this.toColorResponse(color));

    return new ColorListResponse({ colors });
  }
}
