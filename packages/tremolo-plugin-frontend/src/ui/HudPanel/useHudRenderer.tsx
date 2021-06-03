import { ParametersStore } from "../../state/ParametersStore";
import { MutableRefObject, useEffect, useRef } from "react";
import Regl from "regl";
import { autorun } from "mobx";
import { debounce, range } from "lodash";
import { ReglAttributes, ReglProps, ReglUniforms, Vec4 } from "./types";

const NUM_VERTICES = 1000;

export function useHudRenderer(
  parametersStore: ParametersStore,
  setWindowHeight: (value: ((prevState: number) => number) | number) => void
): MutableRefObject<HTMLDivElement | null> {
  const canvasContainerRef = useRef<HTMLDivElement | null>(null);
  const reglRef = useRef<Regl.Regl | null>(null);

  useEffect(() => {
    const canvasEl = canvasContainerRef.current;
    if (!canvasEl) {
      return;
    }

    const regl = Regl({
      container: canvasEl,
    });
    reglRef.current = regl;

    const vertices = range(NUM_VERTICES).map((x) => [
      x / NUM_VERTICES,
      x / NUM_VERTICES,
    ]);
    const drawTriangle = regl<ReglUniforms, ReglAttributes, ReglProps, {}>({
      // Shaders in regl are just strings.  You can use glslify or whatever you want
      // to define them.  No need to manually create shader objects.
      frag: `
        precision mediump float;
        uniform vec4 color;
        void main() {
          gl_FragColor = color;
        }
      `,
      vert: `
        precision mediump float;
        attribute vec2 position;
        uniform float time;
        uniform float depth;
        uniform float phase;
        uniform float rate;

        void main() {
          float PI = 3.14159;

          float x = (position.x - 0.5) * 1.9;

          float depthFactor = (depth / 100.0) * 0.8;
          float phasePrime = position.x * 3. * rate * PI + (phase / 360.0) * 2. * PI;
          float y = depthFactor * sin(phasePrime);
          gl_Position = vec4(
            x,
            y,
            0,
            1
          );
        }
      `,

      attributes: {
        position: regl.prop<ReglProps, "position">("position"),
      },

      uniforms: {
        // This defines the color of the triangle to be a dynamic variable
        color: regl.prop<ReglProps, "color">("color"),
        time: regl.prop<ReglProps, "time">("time"),
        depth: regl.prop<ReglProps, "depth">("depth"),
        phase: regl.prop<ReglProps, "phase">("phase"),
        rate: regl.prop<ReglProps, "rate">("rate"),
        // @ts-ignore
        resolution: ({ viewportWidth, viewportHeight }) => [
          viewportWidth,
          viewportHeight,
        ],
      },

      // This tells regl the number of vertices to draw in this command
      count: NUM_VERTICES,
      primitive: "line strip",
    });

    const clearColor: Vec4 = [24 / 255, 24 / 255, 24 / 255, 1];
    const mainColor: Vec4 = [33 / 255, 170 / 255, 230 / 255, 1];
    const secondaryColor: Vec4 = [200 / 255, 120 / 255, 60 / 255, 1];
    const tick = ({ time }: { time: number }) => {
      regl.clear({
        color: clearColor,
      });

      const depth = parametersStore.depth?.value ?? 100.0;
      const rate = parametersStore.rate?.value ?? 0.1;
      const phase = parametersStore.phase?.value ?? 0.0;
      drawTriangle([
        {
          color: mainColor,
          position: vertices,
          time,
          depth,
          phase: 0,
          rate,
        },
      ]);
      drawTriangle([
        {
          color: secondaryColor,
          position: vertices,
          time,
          depth,
          phase,
          rate,
        },
      ]);
    };

    const dispose = autorun(() => {
      tick({ time: Date.now() });
    });

    const onResize = debounce(() => {
      setWindowHeight(window.innerHeight);
      requestAnimationFrame(() => {
        regl.poll();
      });
    }, 100);
    window.addEventListener("resize", onResize);

    return () => {
      regl.destroy();
      dispose();
      window.removeEventListener("resize", onResize);
    };
  }, [parametersStore, setWindowHeight]);

  return canvasContainerRef;
}
