import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { CqrsModule } from '@nestjs/cqrs';
import { TypeOrmModule } from '@nestjs/typeorm';

import { ApplicationModule } from './application/application.module';
import { getDatabaseConfig } from './config/database.config';
import { DatabaseModule } from './infrastructure';
import { PresentationModule } from './presentation/presentation.module';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    TypeOrmModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: getDatabaseConfig,
      inject: [ConfigService],
    }),
    CqrsModule,
    DatabaseModule,
    ApplicationModule,
    PresentationModule,
  ],
})
export class AppModule {}
