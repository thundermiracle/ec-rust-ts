// eslint-disable-next-line @typescript-eslint/no-unused-vars, @typescript-eslint/no-empty-object-type
export interface IPresenter<TDto, TResponse> {
  // Presenters are implemented as static classes with transformation methods
}

export interface IRequest<TCommand> {
  toCommand(): TCommand;
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export interface IQuery<TResult> {
  // Query interface for type safety - implementations provide toQuery() method
}
