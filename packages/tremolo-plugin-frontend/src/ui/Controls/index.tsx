import React from "react";
import "./index.css";
import { RotaryControl } from "./RotaryControl";
import { ParametersStore } from "../../state/ParametersStore";
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
        {parameters.map((parameter) => (
          <RotaryControl
            key={parameter.id}
            name={parameter.name}
            onChange={(value) => {
              setParameter(parameter.id, value);
            }}
          />
        ))}
      </div>
    </div>
  );
}

export default observer(Controls);
