export interface IRequest<TCommand> {
  toCommand(): TCommand;
}
