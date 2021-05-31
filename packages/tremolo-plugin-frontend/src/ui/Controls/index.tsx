import React from "react";
import "./index.css";
import { RotaryControl } from "./RotaryControl";

export default function Controls() {
  return (
    <div className="Controls">
      <div className="Box">
        <RotaryControl name="Depth" />
        <RotaryControl name="Rate" />
        <RotaryControl name="Figure" />
        <RotaryControl name="Sync" />
        <RotaryControl name="Limit" />
      </div>

      <div className="Box">
        <RotaryControl name="Dry" />
        <RotaryControl name="Wet" />
        <RotaryControl name="Waveform" />
        <RotaryControl name="Smoothing" />
        <RotaryControl name="Offset L" />
        <RotaryControl name="Offset R" />
      </div>
    </div>
  );
}
