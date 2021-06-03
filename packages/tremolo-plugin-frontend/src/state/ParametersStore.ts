import { ParameterDeclarationMessage } from "../common/protocol";
import { singleton } from "tsyringe";
import { makeAutoObservable } from "mobx";
import {PHASE_PARAMETER_ID, DEPTH_PARAMETER_ID, RATE_PARAMETER_ID} from "../common/constants";

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

  get rate(): null | ParameterState {
    return this.parameterValues[RATE_PARAMETER_ID] ?? null;
  }

  get phase(): null | ParameterState {
    return this.parameterValues[PHASE_PARAMETER_ID] ?? null;
  }

  setParameterValue(id: string, value: number) {
    const parameter = this.parameterValues[id];
    parameter.value = value;
  }
}
