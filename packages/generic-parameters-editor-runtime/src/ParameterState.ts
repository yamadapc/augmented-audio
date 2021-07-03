import { makeObservable, observable } from "mobx";

export class ParameterState {
  value: number;

  constructor(value: number) {
    this.value = value;
    makeObservable(this, {
      value: observable,
    });
  }
}
