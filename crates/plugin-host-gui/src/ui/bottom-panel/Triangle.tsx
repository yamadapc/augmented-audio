import React from "react";

interface Props {
  fill: string;
  size: number;
  style?: any;
}

export function Triangle({ fill, size, style }: Props) {
  const height = size;
  const width = size * 0.8;
  return (
    <svg
      style={style}
      height={height}
      width={width}
      viewBox={`0 0 ${width} ${height}`}
    >
      <polygon
        points={`0,0 ${width},${height / 2} 0,${height}`}
        className="triangle"
        fill={fill}
      />
    </svg>
  );
}
