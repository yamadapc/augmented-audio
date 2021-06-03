import "./index.css";
import {useState} from "react";
import {ParametersStore} from "../../state/ParametersStore";
import {observer} from "mobx-react";
import {useHudRenderer} from "./useHudRenderer";

interface Props {
  parametersStore: ParametersStore;
}

function HudPanel({ parametersStore }: Props) {
  const [windowHeight, setWindowHeight] = useState(() => window.innerHeight);
  const canvasContainerRef = useHudRenderer(parametersStore, setWindowHeight);

  return (
    <div className="HudPanel" style={{ position: "relative" }}>
      <div ref={canvasContainerRef} style={{ height: windowHeight - 100 }} />
    </div>
  );
}

export default observer(HudPanel);
