export type Vec4 = [number, number, number, number];

export interface ReglUniforms {
  color: Vec4;
  time: number;
  depth: number;
  phase: number;
  rate: number;
}

export interface ReglAttributes {
  position: number[][];
}

export interface ReglProps {
  position: number[][];
  color: Vec4;
  time: number;
  depth: number;
  phase: number;
  rate: number;
}
