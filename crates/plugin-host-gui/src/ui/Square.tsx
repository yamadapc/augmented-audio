import React from "react";

interface Props {
  fill: string;
  size: number;
}

export function Square({ size, fill }: Props) {
  const height = size;
  const width = size;

  return (
    <svg height={size} width={size} viewBox={`0 0 ${size} ${size}`}>
      <polygon
        points={`0,0 ${width},0 ${width},${height} 0,${height}`}
        className="triangle"
        fill={fill}
      />
    </svg>
  );
}
