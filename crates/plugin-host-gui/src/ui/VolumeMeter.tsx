import React, { useEffect, useRef } from "react";

export function VolumeMeter() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const width = 20;
  const height = 100;

  useEffect(() => {
    let random = Math.random();
    let barHeight = random * height;
    let randomUpdateInterval = setInterval(() => {
      random = Math.random();
      barHeight = random * height;
    }, 100);
    let animation: number | null = null;
    let context = canvasRef.current?.getContext("2d");

    const render = () => {
      const canvas = canvasRef.current;
      if (context == null) {
        context = canvas?.getContext("2d");
      }
      if (context != null) {
        context.save();
        context.clearRect(0, 0, width, height);
        context.fillStyle = "red";
        context.fillRect(0, height - barHeight, width, barHeight);
        context.restore();
      }
      animation = requestAnimationFrame(render);
    };
    animation = requestAnimationFrame(render);

    return () => {
      if (animation != null) {
        cancelAnimationFrame(animation);
      }
      clearInterval(randomUpdateInterval);
    };
  }, [canvasRef]);

  return (
    <div>
      <canvas
        style={{ willChange: "opacity" }}
        width={width}
        height={height}
        ref={canvasRef}
      />
    </div>
  );
}
