import "./index.css";
import Regl from "regl";
import { useEffect, useRef, useState } from "react";
import { ParametersStore } from "../../state/ParametersStore";
import { observer } from "mobx-react";
import { setInterval } from "timers";

const NUM_VERTICES = 1000;

function range(c: number): number[] {
  const r = [];
  for (let i = 0; i < c; i++) {
    r.push(i);
  }
  return r;
}

type Vec4 = [number, number, number, number];

interface ReglUniforms {
  color: Vec4;
  time: number;
  depth: number;
  phase: number;
  rate: number;
}

interface ReglAttributes {
  position: number[][];
}

interface ReglProps {
  position: number[][];
  color: Vec4;
  time: number;
  depth: number;
  phase: number;
  rate: number;
}

interface Props {
  parametersStore: ParametersStore;
}

function HudPanel({ parametersStore }: Props) {
  const [windowHeight, setWindowHeight] = useState(() => window.innerHeight);
  const canvasContainerRef = useRef(null);
  const reglRef = useRef<Regl.Regl | null>(null);
  const stopped = useRef(true);

  useEffect(() => {
    const canvasEl = canvasContainerRef.current;
    if (!canvasEl) {
      return;
    }

    const regl = Regl({
      container: canvasEl,
    });
    reglRef.current = regl;

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
          gl_Position = vec4(
            (position.x - 0.5) * 1.9,
            (depth / 100.0) * 0.8 * sin(position.x * 3. * rate * PI + (phase / 360.0) * 2. * PI),
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
      },

      // This tells regl the number of vertices to draw in this command
      count: NUM_VERTICES,
      primitive: "line strip",
    });

    const vertices = range(NUM_VERTICES).map((x) => [
      x / NUM_VERTICES,
      x / NUM_VERTICES,
    ]);

    const clearColor: Vec4 = [24 / 255, 24 / 255, 24 / 255, 1];
    const mainColor: Vec4 = [33 / 255, 170 / 255, 230 / 255, 1];
    const secondaryColor: Vec4 = [200 / 255, 120 / 255, 60 / 255, 1];
    const tick = ({ time }: { time: number }) => {
      regl.clear({
        color: clearColor,
      });

      const depth = parametersStore.depth?.value ?? 100.0;
      const rate = parametersStore.rate?.value ?? 0.1;
      const timeWithStopped = stopped.current ? 0 : time;
      drawTriangle([
        {
          color: mainColor,
          position: vertices,
          time: timeWithStopped,
          depth,
          phase: 0,
          rate,
        },
      ]);

      const phase = parametersStore.phase?.value ?? 0.0;
      drawTriangle([
        {
          color: secondaryColor,
          position: vertices,
          time: timeWithStopped,
          depth,
          phase,
          rate,
        },
      ]);
    };

    tick({ time: 0 });
    regl.frame(({ time }) => {
      tick({ time });
    });

    const onResize = () => {
      setWindowHeight(window.innerHeight);
      regl.poll();
    };
    window.addEventListener("resize", onResize);

    return () => {
      regl.destroy();
      window.removeEventListener("resize", onResize);
    };
  }, [parametersStore, setWindowHeight]);

  return (
    <div className="HudPanel" style={{ position: "relative" }}>
      <div ref={canvasContainerRef} style={{ height: windowHeight - 100 }} />

      {/*<button*/}
      {/*  onClick={() => {*/}
      {/*    stopped.current = !stopped.current;*/}
      {/*  }}*/}
      {/*  style={{*/}
      {/*    backgroundColor: "#333",*/}
      {/*    color: "white",*/}
      {/*    userSelect: "none",*/}
      {/*    borderRadius: 2,*/}
      {/*    padding: 5,*/}
      {/*    textTransform: "lowercase",*/}
      {/*    lineHeight: 1,*/}
      {/*    border: "solid 1px #666",*/}
      {/*    position: "absolute",*/}
      {/*    bottom: 10,*/}
      {/*    left: 10,*/}
      {/*  }}*/}
      {/*>*/}
      {/*  Start / Stop*/}
      {/*</button>*/}
    </div>
  );
}

export default observer(HudPanel);
