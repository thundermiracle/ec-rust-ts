import {
  Entity,
  Column,
  PrimaryColumn,
  CreateDateColumn,
  UpdateDateColumn,
} from 'typeorm';

@Entity('colors')
export class ColorEntity {
  @PrimaryColumn('integer')
  id: number;

  @Column('text')
  name: string;

  @Column('text')
  hex: string;

  @CreateDateColumn()
  created_at: Date;

  @UpdateDateColumn()
  updated_at: Date;
}
