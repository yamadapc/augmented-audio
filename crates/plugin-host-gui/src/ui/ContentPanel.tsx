import { open as openDialog } from "@tauri-apps/api/dialog";
import React from "react";
import styled from "styled-components";
import { BORDER_COLOR } from "./constants";

const ContentPanelContainer = styled.div({
  flex: 1,
  backgroundColor: "#2a2a2a",
  borderBottom: `solid 1px ${BORDER_COLOR}`,
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
