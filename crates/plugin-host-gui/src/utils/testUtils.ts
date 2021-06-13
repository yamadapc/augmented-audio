export function castMock<Return>(fn: (...args: any[]) => Return): any {
  return fn as any;
}
