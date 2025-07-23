import {
  Entity,
  Column,
  PrimaryColumn,
  ManyToOne,
  OneToMany,
  JoinColumn,
} from 'typeorm';

@Entity('categories')
export class CategoryEntity {
  @PrimaryColumn('text')
  id: string;

  @Column('text')
  name: string;

  @Column('text')
  slug: string;

  @Column('text', { nullable: true })
  parent_id?: string;

  @Column('integer', { default: 0 })
  display_order: number;

  @ManyToOne(() => CategoryEntity, (category) => category.children, {
    nullable: true,
  })
  @JoinColumn({ name: 'parent_id' })
  parent?: CategoryEntity;

  @OneToMany(() => CategoryEntity, (category) => category.parent)
  children: CategoryEntity[];
}
