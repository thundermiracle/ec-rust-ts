import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { GetColorListQuery } from '../../application/queries/models';
import { ColorListDto } from '../../application/dto';

@ApiTags('Colors')
@Controller('colors')
export class ColorsController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get color list',
    description: 'Get all available colors with hex codes',
  })
  @ApiResponse({
    status: 200,
    description: 'Color list retrieved successfully',
    type: ColorListDto,
  })
  async getColorList(): Promise<ColorListDto> {
    const query = new GetColorListQuery();
    return await this.queryBus.execute(query);
  }
}
