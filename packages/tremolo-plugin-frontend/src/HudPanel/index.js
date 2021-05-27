import "./index.css";
import Regl from "regl";
import { useEffect, useRef } from "react";

const NUM_VERTICES = 100;

export default function HudPanel() {
  const canvasContainerRef = useRef(null);
  useEffect(() => {
    if (!canvasContainerRef.current) {
      return;
    }

    const regl = Regl({
      container: canvasContainerRef.current,
    });

    const range = (c) => {
      const r = [];
      for (let i = 0; i < c; i++) {
        r.push(i);
      }
      return r;
    };

    const drawTriangle = regl({
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
        void main() {
          gl_Position = vec4(position.x, position.y, 0, 1);
        }
      `,

      // Here we define the vertex attributes for the above shader
      attributes: {
        // regl.buffer creates a new array buffer object
        position: regl.prop("position"),
        // regl automatically infers sane defaults for the vertex attribute pointers
      },

      uniforms: {
        // This defines the color of the triangle to be a dynamic variable
        color: regl.prop("color"),
      },

      // This tells regl the number of vertices to draw in this command
      count: NUM_VERTICES,
      primitive: "line strip",
    });

    regl.frame(({ time, viewportWidth }) => {
      // clear contents of the drawing buffer
      regl.clear({
        color: [0, 0, 0, 1],
        depth: 1,
      });

      drawTriangle({
        color: [
          33 / 255,
          170,
          (Math.sin(time * 0.1) * 20 + 230) / 255,
          1,
        ],
        position: range(NUM_VERTICES).map((x) => [
          0.1 + 2 * (Math.sin(x / NUM_VERTICES)) - 1.0,
          0.9 * Math.sin((time % 10000) * (x / NUM_VERTICES) * 3 * 2 * Math.PI),
        ]),
      });
    });

    return () => {
      regl.destroy();
    };
  }, []);

  return (
    <div className="HudPanel">
      <div
        ref={canvasContainerRef}
        style={{ height: window.innerHeight - 180 }}
      />
    </div>
  );
}
