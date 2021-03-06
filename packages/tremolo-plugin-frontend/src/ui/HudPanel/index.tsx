import "./index.css";
import { useEffect, useMemo, useState } from "react";
import { ParametersStore } from "@ruas/generic-parameters-editor-runtime/lib/ParametersStore";
import { observer } from "mobx-react";
import { range, throttle } from "lodash";
import { useLogger } from "@wisual/logger";
import {
  DEPTH_PARAMETER_ID,
  PHASE_PARAMETER_ID,
  RATE_PARAMETER_ID,
} from "../../common/constants";

interface Props {
  parametersStore: ParametersStore;
}

function generatePoints(
  windowHeight: number,
  windowWidth: number,
  rateParam: number | null | void,
  depthParam: number | null | void,
  phaseParam: number | null | void
) {
  const height = windowHeight - 100;
  const width = windowWidth;
  const rate = rateParam ?? 440;
  const depth = (depthParam ?? 0) / 100;
  const offset = ((phaseParam ?? 0) / 360) * 2 * Math.PI;
  const numPoints = width / 3;

  const points = range(numPoints).map((i) => {
    const pointPerc = i / numPoints;
    const phase = offset + pointPerc * 10 * rate;
    return [
      pointPerc * (width - 10),
      ((1 + depth * 0.9 * Math.sin(phase)) * height) / 2,
    ];
  });

  return `
    M ${points[0][0]} ${points[0][1]}
    ${points
      .slice(1)
      .map((point, i) => {
        const previousPoint = points[i];
        return `L ${previousPoint.join(" ")} ${point.join(" ")}`;
      })
      .join("\n")}
  `;
}

function HudPanel({ parametersStore }: Props) {
  const logger = useLogger("HudPanel");
  const [windowWidth, setWindowWidth] = useState(() => window.innerWidth);
  const [windowHeight, setWindowHeight] = useState(() => window.innerHeight);
  useEffect(() => {
    const onResize = throttle(() => {
      logger.debug("Window has resized");
      setWindowWidth(window.innerWidth);
      setWindowHeight(window.innerHeight);
    }, 100);
    window.addEventListener("resize", onResize);
    return () => {
      window.removeEventListener("resize", onResize);
    };
  }, [logger]);

  const rateValue = parametersStore.parameterValues[RATE_PARAMETER_ID]?.value;
  const depthValue = parametersStore.parameterValues[DEPTH_PARAMETER_ID]?.value;
  const phaseValue = parametersStore.parameterValues[PHASE_PARAMETER_ID]?.value;

  const leftPoints = useMemo(
    () => generatePoints(windowHeight, windowWidth, rateValue, depthValue, 0),
    [windowHeight, windowWidth, rateValue, depthValue]
  );
  const rightPoints = useMemo(
    () =>
      generatePoints(
        windowHeight,
        windowWidth,
        rateValue,
        depthValue,
        phaseValue
      ),
    [windowHeight, windowWidth, rateValue, depthValue, phaseValue]
  );

  return (
    <div className="HudPanel" style={{ position: "relative" }}>
      <svg
        style={{ height: windowHeight - 100 }}
        viewBox={`0 0 ${window.innerWidth - 5} ${windowHeight - 100}`}
      >
        <path
          fill={"transparent"}
          stroke={"rgb(200, 120, 60)"}
          strokeWidth={1}
          d={rightPoints}
        />
        <path
          fill={"transparent"}
          stroke={"rgb(33, 170, 230)"}
          strokeWidth={1}
          d={leftPoints}
        />
      </svg>
    </div>
  );
}

export default observer(HudPanel);
