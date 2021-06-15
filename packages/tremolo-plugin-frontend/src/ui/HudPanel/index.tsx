import "./index.css";
import { useEffect, useMemo, useState } from "react";
import { ParametersStore } from "../../state/ParametersStore";
import { observer } from "mobx-react";
import { useHudRenderer } from "./useHudRenderer";
import { range } from "lodash";

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
  const [windowWidth, setWindowWidth] = useState(() => window.innerWidth);
  const [windowHeight, setWindowHeight] = useState(() => window.innerHeight);
  useEffect(() => {
    const onResize = () => {
      setWindowWidth(window.innerWidth);
      setWindowHeight(window.innerHeight);
    };
    document.addEventListener("resize", onResize);
    return () => {
      document.removeEventListener("resize", onResize);
    };
  }, []);

  const leftPoints = useMemo(
    () =>
      generatePoints(
        windowHeight,
        windowWidth,
        parametersStore.rate?.value,
        parametersStore.depth?.value,
        0
      ),
    [windowHeight, parametersStore.rate?.value, parametersStore.depth?.value]
  );
  const rightPoints = useMemo(
    () =>
      generatePoints(
        windowHeight,
        windowWidth,
        parametersStore.rate?.value,
        parametersStore.depth?.value,
        parametersStore.phase?.value
      ),
    [
      windowHeight,
      parametersStore.rate?.value,
      parametersStore.depth?.value,
      parametersStore.phase?.value,
    ]
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
