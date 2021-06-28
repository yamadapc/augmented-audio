import React, {useCallback} from "react";
import styled from "styled-components";
import {Triangle} from "./Triangle";
import {Pause} from "./Pause";
import {Square} from "./Square";
import {invoke} from "@tauri-apps/api/tauri";

const TransportContainer = styled.div({
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "100%",
});

const TransportButton = styled.button({
  margin: 0,
  border: 0,
  padding: 0,
  background: "transparent",
  height: 50,
  width: 50,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  transition: "all ease-out 0.1s",

  "&:hover": {
    opacity: 0.7,
  },

  "&:active": {
    opacity: 0.5,
  },
});

export function TransportControls() {
  const onClickPlay = useCallback(() => {
    invoke("play_command");
  }, []);
  const onClickPause = useCallback(() => {
    invoke("pause_command");
  }, []);
  const onClickStop = useCallback(() => {
    invoke("stop_command");
  }, []);

  return (
    <TransportContainer>
      <TransportButton aria-label="Pause" onClick={onClickPause}>
        <Pause fill="white" size={25} />
      </TransportButton>
      <TransportButton aria-label="Play" onClick={onClickPlay}>
        <Triangle fill="white" size={25} />
      </TransportButton>
      <TransportButton aria-label="Stop" onClick={onClickStop}>
        <Square fill="white" size={25} />
      </TransportButton>
    </TransportContainer>
  );
}
