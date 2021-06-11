import React, { useEffect, useState } from "react";
import "./App.css";
import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import { HostSelect } from "./ui/HostSelect";
import { HorizontalLine } from "./ui/HorizontalLine";
import styled from "styled-components";

interface AudioDevice {
  name: string;
}

interface DevicesList {
  inputDevices: AudioDevice[];
  outputDevices: AudioDevice[];
}

interface CommandState<T> {
  loading: boolean;
  data: null | T;
  error: null | Error;
}

interface UseCommandParams {
  skip: boolean;
  args: InvokeArgs;
}

function useCommand<T>(
  command: string,
  params?: UseCommandParams
): CommandState<T> {
  const [loading, setLoading] = useState(!params?.skip);
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const run = async () => {
      setLoading(true);
      const result = await invoke<T>(command, params?.args);
      setData(result);
    };

    run().catch((err) => {
      setError(err);
    });
  }, [command, params?.args]);

  return { loading, data, error };
}

const Header = styled.h1({
  userSelect: "none",
  margin: 0,
  fontSize: 25,
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
      <Header>Audio IO</Header>
      <HorizontalLine />
      <HostSelect label="Audio driver" options={hostOptions} />
      <HostSelect label="Input device" options={inputDeviceOptions} />
      <HostSelect label="Output device" options={outputDeviceOptions} />
      <HorizontalLine />
    </div>
  );
}

export default App;
