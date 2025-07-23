import { Entity, Column, PrimaryColumn } from 'typeorm';

@Entity('colors')
export class ColorEntity {
  @PrimaryColumn('integer')
  id: number;

  @Column('text')
  name: string;

  @Column('text')
  hex: string;
}
