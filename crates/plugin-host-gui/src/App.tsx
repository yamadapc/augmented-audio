import React from "react";
import "./App.css";
import { HostSelect } from "./ui/HostSelect";
import { Header } from "./ui/Header";
import { useCommand } from "./services/useCommand";
import { DevicesList } from "./model";
import { BottomPanel } from "./ui/BottomPanel";
import { ContentPanel } from "./ui/ContentPanel";
import styled from "styled-components";
import { Container } from "./ui/Container";
import {BORDER_COLOR} from "./ui/constants";

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
      value: device,
      label: device.name,
    })) ?? [];
  const outputDeviceOptions =
    devicesList?.outputDevices.map((device) => ({
      value: device,
      label: device.name,
    })) ?? [];

  return (
    <div className="App">
      <AudioIOContainer>
        <HeaderContainer>
          <Header>Audio IO</Header>
        </HeaderContainer>
        <Container>
          <HostSelect label="Audio driver" options={hostOptions} />
          <HostSelect label="Input device" options={inputDeviceOptions} />
          <HostSelect label="Output device" options={outputDeviceOptions} />
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
