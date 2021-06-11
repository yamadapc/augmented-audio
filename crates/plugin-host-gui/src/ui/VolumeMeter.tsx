import React, { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

class InterpolatedValue {
  private value: number;
  private durationMs: number;
  private target: number;
  private step: number;

  constructor(initialValue: number, durationMs: number) {
    this.value = initialValue;
    this.durationMs = durationMs;
    this.target = initialValue;
    this.step = 0;
  }

  get(ticksMs: number = 16) {
    this.value += this.step * ticksMs;
    if (this.step > 0 && this.value >= this.target) {
      this.value = this.target;
      this.step = 0;
    } else if (this.step < 0 && this.value <= this.target) {
      this.value = this.target;
      this.step = 0;
    }
    return this.value;
  }

  setValue(value: number) {
    this.target = value;
    const delta = this.target - this.value;
    this.step = delta / this.durationMs;
  }
}

export function VolumeMeter() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const width = 20;
  const height = 100;

  useEffect(() => {
    let barHeight1 = new InterpolatedValue(0, 100);
    let barHeight2 = new InterpolatedValue(0, 100);
    const barWidth = (width - 2) / 2;
    let animation: number | null = null;
    let context = canvasRef.current?.getContext("2d");
    const subscriptionId = invoke<number>("subscribe_to_volume_command");
    const unlistenPromise = listen<number[]>("volume", (event) => {
      const volume1 = event.payload[0];
      const volume2 = event.payload[1];
      barHeight1.setValue(volume1 * height);
      barHeight2.setValue(volume2 * height);
      if (Math.random() < 0.1) {
        console.log(event);
      }
    });

    const render = () => {
      const canvas = canvasRef.current;
      if (context == null) {
        context = canvas?.getContext("2d");
      }
      if (context != null) {
        context.save();
        context.clearRect(0, 0, width, height);
        context.fillStyle = "#49ee1e";
        const b1 = barHeight1.get(16);
        context.fillRect(0, height - b1, barWidth, b1);
        const b2 = barHeight2.get(16);
        context.fillRect(barWidth + 2, height - b2, barWidth, b2);
        context.restore();
      }
      animation = requestAnimationFrame(render);
    };
    animation = requestAnimationFrame(render);

    return () => {
      if (animation != null) {
        cancelAnimationFrame(animation);
      }
      unlistenPromise.then((unlisten) => unlisten());
      subscriptionId.then((id) =>
        invoke("unsubscribe_to_volume_command", { id })
      );
    };
  }, [canvasRef]);

  return (
    <div
      style={{
        backgroundColor: "#232323",
        display: "inline-block",
        padding: 2,
      }}
    >
      <canvas
        style={{ willChange: "opacity", position: "relative", bottom: -2 }}
        width={width}
        height={height}
        ref={canvasRef}
      />
    </div>
  );
}
