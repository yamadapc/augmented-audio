import { open as openDialog } from "@tauri-apps/api/dialog";
import React from "react";
import styled from "styled-components";

const ContentPanelContainer = styled.div({
  flex: 1,
  backgroundColor: "#2a2a2a",
  borderBottom: "solid 1px black",
  padding: 15,
});

export function ContentPanel() {
  return (
    <ContentPanelContainer>
      <button
        onClick={() => {
          openDialog().then((result) => {
            console.log(result);
          });
        }}
      >
        Select input file
      </button>
    </ContentPanelContainer>
  );
}
