import { ParameterDeclarationMessage } from "./protocol";
import { singleton } from "tsyringe";
import { action, makeObservable, observable, trace } from "mobx";
import { ParameterState } from "./ParameterState";

@singleton()
export class ParametersStore {
  parameters: ParameterDeclarationMessage[] = [];
  parameterValues: { [parameterId: string]: ParameterState } = {};

  constructor() {
    makeObservable(this, {
      parameters: observable,
      parameterValues: observable,
      setParameterValue: action,
      setParameters: action,
    });
  }

  setParameters(parameters: ParameterDeclarationMessage[]) {
    this.parameters = parameters;
    this.parameterValues = {};
    parameters.forEach((parameter) => {
      this.parameterValues[parameter.id] = new ParameterState(parameter.value);
    });
  }

  setParameterValue(id: string, value: number) {
    const parameter = this.parameterValues[id];
    parameter.value = value;
  }
}
