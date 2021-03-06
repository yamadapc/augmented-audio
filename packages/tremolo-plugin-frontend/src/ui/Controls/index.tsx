import React from "react";
import "./index.css";
import RotaryControl from "./RotaryControl";
import { ParametersStore } from "@ruas/generic-parameters-editor-runtime/lib/ParametersStore";
import { observer } from "mobx-react";

interface Props {
  parametersStore: ParametersStore;
  setParameter: (id: string, value: number) => void;
}

export function Controls({ parametersStore, setParameter }: Props) {
  const parameters = parametersStore.parameters;

  return (
    <div className="Controls">
      <div className="Box">
        {parameters.map((parameter) => {
          const state = parametersStore.parameterValues[parameter.id];
          if (!state) {
            return null;
          }
          return (
            <RotaryControl
              key={parameter.id}
              declaration={parameter}
              state={state}
              onChange={(id, val) => {
                setParameter(id, val);
              }}
            />
          );
        })}
      </div>
    </div>
  );
}

export default observer(Controls);
