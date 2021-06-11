import React from "react";
import "./App.css";
import { open as openDialog } from "@tauri-apps/api/dialog";
import { HostSelect } from "./ui/HostSelect";
import { HorizontalLine } from "./ui/HorizontalLine";
import { VolumeMeter } from "./ui/VolumeMeter";
import { Header } from "./ui/Header";
import { useCommand } from "./services/useCommand";
import { DevicesList } from "./model";

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
      <Header>Audio IO</Header>
      <HorizontalLine />
      <HostSelect label="Audio driver" options={hostOptions} />
      <HostSelect label="Input device" options={inputDeviceOptions} />
      <HostSelect label="Output device" options={outputDeviceOptions} />
      <HorizontalLine />

      <VolumeMeter />

      <button
        onClick={() => {
          openDialog().then((result) => {
            console.log(result);
          });
        }}
      >
        Select input file
      </button>
    </div>
  );
}

export default App;
