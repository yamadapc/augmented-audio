import React from "react";
import "./App.css";
import {BottomPanel} from "./ui/bottom-panel/BottomPanel";
import {ContentPanel} from "./ui/content/ContentPanel";
import styled from "styled-components";
import {AudioIOOptions} from "./ui/audio-io-options/AudioIOOptions";
import StatusBar from "./ui/status-bar/StatusBar";

const BodyLayoutContainer = styled.div({
  display: "flex",
  flex: 1,
  flexDirection: "column",
});

function App() {
  return (
    <div className="App">
      <AudioIOOptions />

      <BodyLayoutContainer>
        <ContentPanel />
        <BottomPanel />
        <StatusBar />
      </BodyLayoutContainer>
    </div>
  );
}

export default App;
