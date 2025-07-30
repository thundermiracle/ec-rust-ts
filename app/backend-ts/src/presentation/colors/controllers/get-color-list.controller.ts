import { Controller, Get } from '@nestjs/common';
import { QueryBus } from '@nestjs/cqrs';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';

import { ColorListDto } from '$application/dto';
import { GetColorListQuery } from '$application/queries/models';

import { ColorsPresenter } from '../presenters';
import { ColorListResponse } from '../responses';

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
