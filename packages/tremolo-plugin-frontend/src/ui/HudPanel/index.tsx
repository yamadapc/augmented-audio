import "./index.css";
import Regl from "regl";
import { useEffect, useRef, useState } from "react";
import { ParametersStore } from "../../state/ParametersStore";
import { observer } from "mobx-react";

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
}

interface ReglAttributes {
  position: number[][];
}

interface ReglProps {
  position: number[][];
  color: Vec4;
  time: number;
  depth: number;
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

      // .map((x) => [
      //   0.1 + 2 * (Math.sin(x / NUM_VERTICES)) - 1.0,
      //   0.9 * Math.sin((time * 4) + (x / NUM_VERTICES) * 3 * 2 * Math.PI),
      // ]),

      vert: `
        precision mediump float;
        attribute vec2 position;
        uniform float time;
        uniform float depth;

        void main() {
          gl_Position = vec4(
            (position.x - 0.5) * 1.9,
            depth * 0.8 * sin(time * 5. + position.x * 30.),
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
      },

      // This tells regl the number of vertices to draw in this command
      count: NUM_VERTICES,
      primitive: "line strip",
    });

    const vertices = range(NUM_VERTICES).map((x) => [
      x / NUM_VERTICES,
      x / NUM_VERTICES,
    ]);

    const tick = ({ time }: { time: number }) => {
      regl.clear({
        color: [24 / 255, 24 / 255, 24 / 255, 1],
        depth: 1,
      });

      drawTriangle([
        {
          color: [33 / 255, 170 / 255, 230 / 255, 1],
          position: vertices,
          time: stopped.current ? 0 : time,
          depth: parametersStore.depth?.value ?? 1.0,
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

      <button
        onClick={() => {
          stopped.current = !stopped.current;
        }}
        style={{
          backgroundColor: "#333",
          color: "white",
          userSelect: "none",
          borderRadius: 2,
          padding: 5,
          textTransform: "lowercase",
          lineHeight: 1,
          border: "solid 1px #666",
          position: "absolute",
          bottom: 10,
          left: 10,
        }}
      >
        Start / Stop
      </button>
    </div>
  );
}

export default observer(HudPanel);
