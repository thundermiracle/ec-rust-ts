import { ConfigService } from '@nestjs/config';
import { TypeOrmModuleOptions } from '@nestjs/typeorm';

export const getDatabaseConfig = (
  configService: ConfigService,
): TypeOrmModuleOptions => ({
  type: 'sqlite',
  database:
    configService.get<string>('DATABASE_PATH', '../backend/data/db.sqlite') ??
    '../backend/data/db.sqlite',
  autoLoadEntities: true,
  synchronize: configService.get<boolean>('DB_SYNC', false) ?? false,
  logging: configService.get<boolean>('DB_LOGGING', true) ?? true,
});
