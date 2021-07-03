import { ParameterDeclarationMessage } from "./protocol";
import { singleton } from "tsyringe";
import { makeAutoObservable } from "mobx";
import { ParameterState } from "./ParameterState";

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

  getParameter(id: string): null | ParameterState {
    return this.parameterValues[id] ?? null;
  }

  setParameterValue(id: string, value: number) {
    const parameter = this.parameterValues[id];
    parameter.value = value;
  }
}
