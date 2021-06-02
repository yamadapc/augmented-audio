import { ParameterDeclarationMessage } from "../common/protocol";
import { singleton } from "tsyringe";
import { makeAutoObservable } from "mobx";
import { DEPTH_PARAMETER_ID } from "../common/constants";

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

@singleton()
export class ParametersStore {
  parameters: ParameterDeclarationMessage[] = [];
  private parameterValues: { [parameterId: string]: ParameterState } = {};

  constructor() {
    makeAutoObservable(this);
  }

  setParameters(parameters: ParameterDeclarationMessage[]) {
    this.parameters = parameters;
    this.parameterValues = {};
    parameters.forEach((parameter) => {
      this.parameterValues[parameter.id] = new ParameterState(parameter.value);
    });
  }

  get depth(): null | ParameterState {
    return this.parameterValues[DEPTH_PARAMETER_ID] ?? null;
  }

  setParameterValue(id: string, value: number) {
    const parameter = this.parameterValues[id];
    parameter.value = value;
  }
}
