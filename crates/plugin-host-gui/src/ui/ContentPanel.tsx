import { open as openDialog } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import React from "react";
import styled from "styled-components";
import { BORDER_COLOR } from "./constants";
import { useLogger } from "@wisual/logger";

const ContentPanelContainer = styled.div({
  flex: 1,
  backgroundColor: "#2a2a2a",
  borderBottom: `solid 1px ${BORDER_COLOR}`,
  padding: 15,
});

export function ContentPanel() {
  const logger = useLogger("ContentPanel");
  return (
    <ContentPanelContainer>
      <button
        onClick={() => {
          logger.info("Opening file dialog");
          openDialog().then((result) => {
            logger.info("Invoking set_input_file_command", {
              inputFile: result,
            });
            invoke("set_input_file_command", { inputFile: result }).catch(
              (err) => logger.error(err)
            );
          });
        }}
      >
        Select input audio file
      </button>
      <hr />
      <button
        onClick={() => {
          logger.info("Opening file dialog");
          openDialog().then((result) => {
            logger.info("Invoking set_plugin_path_comment", {
              path: result,
            });
            invoke("set_plugin_path_command", { path: result }).catch(
              (err) => logger.error(err)
            );
          });
        }}
      >
        Select plugin path
      </button>
    </ContentPanelContainer>
  );
}
