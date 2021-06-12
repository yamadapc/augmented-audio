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
  const width = 20;
  const height = 100;
  const barWidth = (width - 4) / 2;
  const boxLeft = useRef<HTMLDivElement | null>(null);
  const boxRight = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    let animation: number | null = null;
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
    }, 100);

    const draw = (box: HTMLDivElement, volume: number) => {
      box.style.transform = `scaleY(${volume})`;
    };

    return () => {
      if (animation != null) {
        cancelAnimationFrame(animation);
      }
      subscriptionId.then((id) =>
        invoke("unsubscribe_to_volume_command", { id })
      );
    };
  }, []);

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
          contentVisibility: "auto",
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
          contentVisibility: "auto",
          transform: "scaleY(0)",
        }}
        ref={boxRight}
      />
    </div>
  );
}
