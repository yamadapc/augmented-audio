import React from "react";

interface Props {
  fill: string;
  size: number;
}

export function Pause({ size, fill }: Props) {
  const height = size;
  const width = size;
  const padding = size * 0.3;
  const midPoint = width / 2;
  const midPointBefore = midPoint - padding / 2;
  const midPointAfter = midPoint + padding / 2;

  return (
    <svg height={size} width={size} viewBox={`0 0 ${size} ${size}`}>
      <polygon
        points={`0,0 ${midPointBefore},0 ${midPointBefore},${height} 0,${height}`}
        className="triangle"
        fill={fill}
      />
      <polygon
        points={`${midPointAfter},0 ${width},0 ${width},${height} ${midPointAfter},${height}`}
        className="triangle"
        fill={fill}
      />
    </svg>
  );
}
