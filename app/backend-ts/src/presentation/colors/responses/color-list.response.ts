import { ApiProperty } from '@nestjs/swagger';

export class ColorResponse {
  @ApiProperty({ description: 'Color ID' })
  id: number;

  @ApiProperty({ description: 'Color name' })
  name: string;

  @ApiProperty({ description: 'Hex color code' })
  hex: string;

  constructor(data: { id: number; name: string; hex: string }) {
    this.id = data.id;
    this.name = data.name;
    this.hex = data.hex;
  }
}

export class ColorListResponse {
  @ApiProperty({
    description: 'List of colors',
    type: [ColorResponse],
  })
  colors: ColorResponse[];

  constructor(data: { colors: ColorResponse[] }) {
    this.colors = data.colors;
  }
}
