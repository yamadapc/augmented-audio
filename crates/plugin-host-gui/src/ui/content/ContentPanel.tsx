import {open as openDialog} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api/tauri";
import React from "react";
import styled from "styled-components";
import {BLACK, BORDER_COLOR, GRAY} from "../constants";
import {useLogger} from "@wisual/logger";

const ContentPanelContainer = styled.div({
  flex: 1,
  backgroundColor: GRAY,
  borderBottom: `solid 1px ${BORDER_COLOR}`,
  padding: 15,
});

const ButtonGroup = styled.div({
  flexDirection: "row",
  display: "flex",
});

const Button = styled.button({
  border: `solid 1px ${BORDER_COLOR}`,
  backgroundColor: BLACK,
  color: "white",
  fontSize: 16,
  padding: 10,
  width: "100%",
});

export function ContentPanel() {
  const logger = useLogger("ContentPanel");

  return (
    <ContentPanelContainer>
      <ButtonGroup>
        <Button
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
        </Button>
        <Button
          onClick={() => {
            logger.info("Opening file dialog");
            openDialog().then((result) => {
              logger.info("Invoking set_plugin_path_comment", {
                path: result,
              });
              invoke("set_plugin_path_command", { path: result }).catch((err) =>
                logger.error(err)
              );
            });
          }}
        >
          Select plugin path
        </Button>
      </ButtonGroup>
    </ContentPanelContainer>
  );
}
