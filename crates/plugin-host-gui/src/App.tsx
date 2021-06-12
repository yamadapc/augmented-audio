import React, { useState } from "react";
import "./App.css";
import { HostSelect } from "./ui/HostSelect";
import { Header } from "./ui/Header";
import { useCommand } from "./services/useCommand";
import { DevicesList } from "./model";
import { BottomPanel } from "./ui/BottomPanel";
import { ContentPanel } from "./ui/ContentPanel";
import styled from "styled-components";
import { Container } from "./ui/Container";
import { BORDER_COLOR } from "./ui/constants";
import { invoke } from "@tauri-apps/api/tauri";

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
  const { data: devicesList } = useCommand<DevicesList>("list_devices_command");
  const { data: hostList } = useCommand<string[]>("list_hosts_command");
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
  // @ts-ignore
  const [host, setHost] = useState();
  // @ts-ignore
  const [inputDevice, setInputDevice] = useState();
  // @ts-ignore
  const [outputDevice, setOutputDevice] = useState();

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
            onChange={(v) => {
              // @ts-ignore
              setHost(v?.value);
              invoke("set_audio_driver_command", { hostId: v?.value }).catch(
                (err) => console.error(err)
              );
            }}
          />
          <HostSelect
            label="Input device"
            options={inputDeviceOptions}
            value={inputDevice}
            onChange={(v) => {
              console.log(v);
              // @ts-ignore
              setInputDevice(v?.value);
              invoke("set_input_device_command", {
                inputDeviceId: v?.value,
              }).catch((err) => console.error(err));
            }}
          />
          <HostSelect
            label="Output device"
            options={outputDeviceOptions}
            value={outputDevice}
            onChange={(v) => {
              // @ts-ignore
              setOutputDevice(v?.value);
              invoke("set_output_device_command", {
                outputDeviceId: v?.value,
              }).catch((err) => console.error(err));
            }}
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
