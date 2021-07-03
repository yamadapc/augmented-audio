import { makeAutoObservable } from "mobx";

export class ParameterState {
  private valueInner: number;

  constructor(value: number) {
    makeAutoObservable(this);
    this.valueInner = value;
  }

  get value(): number {
    return this.valueInner;
  }

  set value(value: number) {
    this.valueInner = value;
  }
}
