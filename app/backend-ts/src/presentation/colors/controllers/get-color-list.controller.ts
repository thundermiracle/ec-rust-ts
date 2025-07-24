import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiTags, ApiOperation, ApiResponse } from '@nestjs/swagger';
import { ColorListResponse } from '../responses';
import { ColorsPresenter } from '../presenters';
import { GetColorListQuery } from '../../../application/queries/models';
import { ColorListDto } from '../../../application/dto';

@ApiTags('Colors')
@Controller('colors')
export class GetColorListController {
  constructor(private readonly queryBus: QueryBus) {}

  @Get()
  @ApiOperation({
    summary: 'Get color list',
    description: 'Get all available colors with hex codes',
  })
  @ApiResponse({
    status: 200,
    description: 'Color list retrieved successfully',
    type: ColorListResponse,
  })
  async execute(): Promise<ColorListResponse> {
    const query = new GetColorListQuery();
    const result = await this.queryBus.execute<GetColorListQuery, ColorListDto>(
      query,
    );
    return ColorsPresenter.toColorListResponse(result);
  }
}
