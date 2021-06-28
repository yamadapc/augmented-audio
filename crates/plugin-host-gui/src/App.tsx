import React, {useCallback, useState} from "react";
import "./App.css";
import {HostSelect, OptionType} from "./ui/common/HostSelect";
import {Header} from "./ui/header/Header";
import {useCommandQuery} from "./services/useCommandQuery";
import {DevicesList} from "./model";
import {BottomPanel} from "./ui/bottom-panel/BottomPanel";
import {ContentPanel} from "./ui/content/ContentPanel";
import styled from "styled-components";
import {Container} from "./ui/Container";
import {BORDER_COLOR} from "./ui/constants";
import {invoke} from "@tauri-apps/api/tauri";
import {useLogger} from "@wisual/logger";

const BodyLayoutContainer = styled.div({
  display: "flex",
  flex: 1,
  flexDirection: "column",
});

const AudioIOContainer = styled.div({
  borderBottom: `solid 1px ${BORDER_COLOR}`,
});

const HeaderContainer = styled(Container)({
  backgroundColor: "#232323",
  borderBottom: `solid 1px ${BORDER_COLOR}`,
});

function App() {
  const logger = useLogger("App");
  const { data: devicesList } = useCommandQuery<DevicesList>(
    "list_devices_command"
  );
  const { data: hostList } = useCommandQuery<string[]>("list_hosts_command");
  const hostOptions =
    hostList?.map((host) => ({ value: host, label: host })) ?? [];
  const inputDeviceOptions =
    devicesList?.inputDevices.map((device) => ({
      value: device.name,
      label: device.name,
    })) ?? [];
  const outputDeviceOptions =
    devicesList?.outputDevices.map((device) => ({
      value: device.name,
      label: device.name,
    })) ?? [];
  const [host, setHost] = useState<string | null>(null);
  const [inputDevice, setInputDevice] = useState<string | null>(null);
  const [outputDevice, setOutputDevice] = useState<string | null>(null);

  const onChangeInputDevice = useCallback(
    (option: OptionType<string> | null) => {
      const value = option?.value;
      if (value == null) {
        return;
      }
      setInputDevice(value);
      invoke("set_input_device_command", {
        inputDeviceId: value,
      }).catch((err) => logger.error(err));
    },
    [setInputDevice, logger]
  );
  const onChangeOutputDevice = useCallback(
    (option: OptionType<string> | null) => {
      const value = option?.value;
      if (value == null) {
        return;
      }
      setOutputDevice(value);
      invoke("set_output_device_command", {
        outputDeviceId: value,
      }).catch((err) => logger.error(err));
    },
    [setOutputDevice, logger]
  );
  const onChangeAudioDriver = useCallback(
    (option: OptionType<string> | null) => {
      const value = option?.value;
      if (value == null) {
        return;
      }
      setHost(value);
      invoke("set_audio_driver_command", { hostId: value }).catch((err) =>
        logger.error(err)
      );
    },
    [setHost, logger]
  );

  return (
    <div className="App">
      <AudioIOContainer>
        <HeaderContainer>
          <Header>Audio IO</Header>
        </HeaderContainer>
        <Container>
          <HostSelect
            label="Audio driver"
            options={hostOptions}
            value={host}
            onChange={onChangeAudioDriver}
          />
          <HostSelect
            label="Input device"
            options={inputDeviceOptions}
            value={inputDevice}
            onChange={onChangeInputDevice}
          />
          <HostSelect
            label="Output device"
            options={outputDeviceOptions}
            value={outputDevice}
            onChange={onChangeOutputDevice}
          />
        </Container>
      </AudioIOContainer>

      <BodyLayoutContainer>
        <ContentPanel />
        <BottomPanel />
      </BodyLayoutContainer>
    </div>
  );
}

export default App;
