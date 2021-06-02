import "./index.css";
import { useCallback, useEffect, useRef, useState } from "react";

const TWO_PI = 2 * Math.PI;

interface Props {
  name: string;
  onChange: (value: number) => void;
}

export function RotaryControl({ name, onChange }: Props) {
  const props = {
    initialValue: 0.4,
  };

  const { initialValue } = props;

  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  const [isMoving, setIsMoving] = useState(false);
  const [value, setValueInner] = useState(initialValue);
  const cleanUpTasks = useRef<(() => void)[]>([]);

  const setValue = useCallback(
    (val) => {
      setValueInner(val);
      onChange(val);
    },
    [setValueInner, onChange]
  );

  const onMouseDown = (e: React.MouseEvent) => {
    setIsMoving(true);
    const start = { x: e.clientX, y: e.clientY };
    setMousePosition(start);

    const listener = (e: MouseEvent) => {
      const current = { x: e.clientX, y: e.clientY };
      setMousePosition(current);
      const deltaX = current.x - start.x;
      const deltaY = -(current.y - start.y);
      const totalMovement = deltaX + deltaY;
      const movementRatio = totalMovement / 100;

      setValue(Math.min(Math.max(0.0, value + movementRatio), 1.0));
    };

    document.addEventListener("mousemove", listener);
    const runCleanUp = () => {
      document.removeEventListener("mousemove", listener);
      document.removeEventListener("mouseup", runCleanUp);
    };

    const onMouseUp = (e: MouseEvent) => {
      setIsMoving(false);
      runCleanUp();
    };
    document.addEventListener("mouseup", onMouseUp);

    cleanUpTasks.current.push(runCleanUp);
  };

  useEffect(() => {
    return () => {
      // eslint-disable-next-line
      cleanUpTasks.current.forEach((task) => task());
    };
  }, []);

  const radius = 40;
  const perimeter = TWO_PI * radius;
  const strokeDashoffset = 0.25 * perimeter;
  const strokeDasharray = perimeter;

  const centerX = 50;
  const centerY = 50;

  const start = TWO_PI * 0.25;
  const end = TWO_PI * 0.25 + TWO_PI * 0.75 * value;
  const startCoords = [
    centerX + radius * Math.cos(start),
    centerY + radius * Math.sin(start),
  ];
  const valueCoords = [
    centerX + radius * Math.cos(end),
    centerY + radius * Math.sin(end),
  ];
  const largeArcFlag = end - start <= Math.PI ? 0 : 1;

  return (
    <div className="RotaryControl" onMouseDown={onMouseDown}>
      <svg
        className="RotaryControl__Circle"
        width="50%"
        height="50%"
        viewBox="0 0 100 100"
      >
        <circle
          cx={centerX}
          cy={centerY}
          className="RotaryControl__Circle__Background"
          r={radius}
          strokeWidth={9}
          strokeDasharray={strokeDasharray}
          strokeDashoffset={strokeDashoffset}
        />

        <path
          d={`
            M ${startCoords.join(" ")}
            A ${radius} ${radius} 0 ${largeArcFlag} 1 ${valueCoords.join(" ")}
          `}
          className="RotaryControl__Circle__Value"
        />

        <circle
          className="RotaryControl__Circle__Knob"
          cx={valueCoords[0]}
          cy={valueCoords[1]}
          r={10}
        />
      </svg>

      <label>{name}</label>

      {isMoving && (
        <div
          style={{
            background: "white",
            opacity: 0.8,
            position: "fixed",
            padding: 5,
            left: mousePosition.x + 10,
            top: mousePosition.y - 20,
            color: "black",
            border: "solid 1px black",
            fontSize: 14,
            zIndex: 10,
          }}
        >
          {value.toFixed(2)}
        </div>
      )}
    </div>
  );
}
