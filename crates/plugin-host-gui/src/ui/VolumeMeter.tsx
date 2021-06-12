import React, { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { BORDER_COLOR } from "./constants";

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

  getTarget() {
    return this.value + this.step * 32;
  }
}

export function VolumeMeter() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const width = 20;
  const height = 100;
  const barWidth = (width - 4) / 2;
  const boxLeft = useRef<HTMLDivElement | null>(null);
  const boxRight = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    let barHeight1 = new InterpolatedValue(0, 100);
    let barHeight2 = new InterpolatedValue(0, 100);
    const barWidth = (width - 4) / 2;
    let animation: number | null = null;
    let context = canvasRef.current?.getContext("2d");
    const subscriptionId = invoke<number>("subscribe_to_volume_command");
    setInterval(() => {
      // @ts-ignore
      const volume1 = window.volume1;
      // @ts-ignore
      const volume2 = window.volume2;
      if (animation == null) {
        animation = requestAnimationFrame(() => {
          if (boxLeft.current) draw(boxLeft.current, volume1);
          if (boxRight.current) draw(boxRight.current, volume2);
          animation = null;
        });
      }
      // barHeight1.setValue(volume1 * height);
      // barHeight2.setValue(volume2 * height);
    }, 100);

    const draw = (box: HTMLDivElement, volume: number) => {
      box.style.transform = `scaleY(${volume})`;
    };
    const render = () => {
      // if (boxLeft.current) draw(boxLeft.current, barHeight1.get());
      // if (boxRight.current) draw(boxRight.current, barHeight2.get());
      // const canvas = canvasRef.current;
      // if (context == null) {
      //   context = canvas?.getContext("2d");
      // }
      // if (context != null) {
      //   context.save();
      //   context.clearRect(0, 0, width, height);
      //
      //   context.fillStyle = "rgba(89,182,71,0.41)";
      //   const b1Prev = barHeight1.getTarget();
      //   context.fillRect(0, height - b1Prev, barWidth, b1Prev);
      //   context.fillStyle = "#59b647";
      //   context.fillRect(0, height - b1Prev - 3, barWidth, 2);
      //   const b1 = barHeight1.get(16);
      //   context.fillRect(0, height - b1, barWidth, b1);
      //
      //   context.fillStyle = "rgba(89,182,71,0.41)";
      //   const b2Prev = barHeight2.getTarget();
      //   context.fillRect(barWidth + 4, height - b2Prev, barWidth, b2Prev);
      //   context.fillStyle = "#59b647";
      //   context.fillRect(barWidth + 4, height - b2Prev - 3, barWidth, 2);
      //   const b2 = barHeight2.get(16);
      //   context.fillRect(barWidth + 4, height - b2, barWidth, b2);
      //
      //   context.restore();
      // }
      // animation = requestAnimationFrame(render);
    };
    // animation = requestAnimationFrame(render);

    return () => {
      if (animation != null) {
        cancelAnimationFrame(animation);
      }
      // unlistenPromise.then((unlisten) => unlisten());
      subscriptionId.then((id) =>
        invoke("unsubscribe_to_volume_command", { id })
      );
    };
  }, [canvasRef]);

  return (
    <div
      style={{
        backgroundColor: "#262626",
        border: `solid 1px ${BORDER_COLOR}`,
        height: height,
        display: "inline-flex",
        padding: `1px 4px`,
        contentVisibility: "auto",
      }}
    >
      <div
        ref={boxLeft}
        style={{
          backgroundColor: "#59b647",
          height: height,
          width: barWidth,
          willChange: "transform",
          transformOrigin: "bottom left",
          transition: "transform 100ms linear",
          transform: "scaleY(0)",
        }}
      />
      <div style={{ width: 5 }} />
      <div
        style={{
          backgroundColor: "#59b647",
          height: height,
          width: barWidth,
          willChange: "transform",
          transformOrigin: "bottom left",
          transition: "transform 100ms linear",
          transform: "scaleY(0)",
        }}
        ref={boxRight}
      />

      {/*<canvas*/}
      {/*  style={{ willChange: "opacity", position: "relative", bottom: -2 }}*/}
      {/*  width={width}*/}
      {/*  height={height}*/}
      {/*  ref={canvasRef}*/}
      {/*/>*/}
    </div>
  );
}
