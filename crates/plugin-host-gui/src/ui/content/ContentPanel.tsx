import {open as openDialog} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api/tauri";
import React from "react";
import styled from "styled-components";
import {BLACK, BORDER_COLOR, GRAY} from "../constants";
import {useLogger} from "@wisual/logger";
import {useCommandQuery} from "../../services/useCommandQuery";
import {HostState} from "../../model";

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

const ButtonContainer = styled.div({
  flex: 1,
  flexDirection: "column",
  display: "flex",
});

const Button = styled.button({
  border: `solid 1px ${BORDER_COLOR}`,
  backgroundColor: BLACK,
  color: "white",
  fontSize: 16,
  padding: 10,
  width: "100%",
  transition: "all ease-out 0.1s",
  "&:hover": {
    opacity: 0.7,
  },
  "&:active": {
    opacity: 0.5,
  },
});

export function ContentPanel() {
  const logger = useLogger("ContentPanel");
  const { data: hostOptions, reload } = useCommandQuery<HostState>(
    "get_host_state_command"
  );

  const audioInputFilePath = hostOptions?.audioInputFilePath;
  const pluginPath = hostOptions?.pluginPath;

  const onClickOpenAudioFile = () => {
    const run = async () => {
      logger.info("Opening file dialog");
      const result = await openDialog();
      logger.info("Invoking set_input_file_command", {
        inputFile: result,
      });
      await invoke("set_input_file_command", { inputFile: result });
      reload();
    };
    run().catch((err) => logger.error(err));
  };

  const onClickSelectPluginPath = () => {
    const run = async () => {
      logger.info("Opening file dialog");
      const result = await openDialog();
      logger.info("Invoking set_plugin_path_comment", {
        path: result,
      });
      await invoke("set_plugin_path_command", { path: result });
      reload();
    };
    run().catch((err) => logger.error(err));
  };

  return (
    <ContentPanelContainer>
      <ButtonGroup>
        <ButtonContainer>
          <Button onClick={onClickOpenAudioFile}>
            Select input audio file
          </Button>
          <code>{audioInputFilePath}</code>
        </ButtonContainer>
        <ButtonContainer>
          <Button onClick={onClickSelectPluginPath}>Select plugin path</Button>
          <code>{pluginPath}</code>
        </ButtonContainer>
      </ButtonGroup>
    </ContentPanelContainer>
  );
}
