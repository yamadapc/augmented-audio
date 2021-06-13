import React, { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { BORDER_COLOR } from "./constants";

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
