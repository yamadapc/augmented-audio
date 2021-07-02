import React, {useCallback, useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {BORDER_COLOR, GREEN, MEDIUM_GRAY} from "../constants";
import {Triangle} from "./Triangle";
import {useLogger} from "@wisual/logger";

export function VolumeMeter() {
  const width = 20;
  const height = 100;
  const barWidth = (width - 4) / 2;
  const boxLeft = useRef<HTMLDivElement | null>(null);
  const boxRight = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    let animation: number | null = null;
    const subscriptionId = invoke<number>("subscribe_to_volume_command");
    const interval = setInterval(() => {
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
      box.style.transform = `scaleY(${Math.min(volume * 9, 1.0)})`;
    };

    return () => {
      clearInterval(interval);
      if (animation != null) {
        cancelAnimationFrame(animation);
      }
      subscriptionId.then((id) =>
        invoke("unsubscribe_to_volume_command", { subscriberId: id })
      );
    };
  }, []);

  const [volumeHandlePos, setVolumeHandlePos] = useState(0);

  const logger = useLogger("VolumeMeter");
  const onDragVolumeHandle = useCallback(() => {
    logger.info("Initializing mouse movement subscription");
    const onMouseMove = (e: MouseEvent) => {
      setVolumeHandlePos((pos) =>
        Math.min(height, Math.max(pos + e.movementY, 0))
      );
    };
    document.addEventListener("mousemove", onMouseMove);

    const onMouseUp = () => {
      logger.info("Cleaning-up mouse movement subscriptions");
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
    };

    document.addEventListener("mouseup", onMouseUp);
  }, [setVolumeHandlePos, logger]);

  useEffect(() => {
    const volume = 1 - volumeHandlePos / height;
    // TODO - Submit volume up, maintain state & implement volume in TestPluginHost
  }, [volumeHandlePos, height]);

  return (
    <div
      style={{
        position: "relative",
        backgroundColor: MEDIUM_GRAY,
        border: `solid 1px ${BORDER_COLOR}`,
        height: height,
        display: "inline-flex",
        padding: `1px 4px`,
        contain: "layout",
      }}
    >
      <div
        ref={boxLeft}
        style={{
          backgroundColor: GREEN,
          height: height,
          width: barWidth,
          willChange: "transform",
          transformOrigin: "bottom left",
          transition: "transform 100ms linear",
          contain: "layout",
          transform: "scaleY(0)",
        }}
      />
      <div style={{ width: 5 }} />
      <div
        style={{
          backgroundColor: GREEN,
          height: height,
          width: barWidth,
          willChange: "transform",
          transformOrigin: "bottom left",
          transition: "transform 100ms linear",
          contain: "layout",
          transform: "scaleY(0)",
        }}
        ref={boxRight}
      />

      <div
        onMouseDown={onDragVolumeHandle}
        style={{
          position: "absolute",
          top: 0,
          left: -2,
          width: "100%",
          cursor: "pointer",
          transform: `translateY(${volumeHandlePos - 8}px)`,
        }}
      >
        <div
          style={{
            backgroundColor: BORDER_COLOR,
            height: 1,
            width: "110%",
            position: "absolute",
            zIndex: 0,
            top: "50%",
          }}
        />
        <Triangle
          style={{ position: "relative", zIndex: 1 }}
          fill="white"
          size={10}
        />
      </div>
    </div>
  );
}
