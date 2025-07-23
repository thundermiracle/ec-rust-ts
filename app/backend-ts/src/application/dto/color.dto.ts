export class ColorDto {
  constructor(
    public readonly id: number,
    public readonly name: string,
    public readonly hex: string,
  ) {}
}

export class ColorListDto {
  constructor(public readonly colors: ColorDto[]) {}
}
