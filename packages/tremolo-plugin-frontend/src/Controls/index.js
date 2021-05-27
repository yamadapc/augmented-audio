import "./index.css";
import { RotaryControl } from "./RotaryControl";

export default function Controls() {
  return (
    <div className="Controls">
      <div className="Box">
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
      </div>

      <div className="Box">
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
        <RotaryControl />
      </div>
    </div>
  );
}
