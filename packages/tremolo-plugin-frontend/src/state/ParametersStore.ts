import { ParameterDeclarationMessage } from "../common/protocol";
import { singleton } from "tsyringe";
import { makeAutoObservable } from "mobx";

@singleton()
export class ParametersStore {
  parameters: ParameterDeclarationMessage[] = [];
  private parameterValues: { [parameterId: string]: number } = {};

  constructor() {
    makeAutoObservable(this);
  }

  setParameters(parameters: ParameterDeclarationMessage[]) {
    this.parameters = parameters;
  }
}
