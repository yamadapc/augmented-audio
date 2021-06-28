import React, {useCallback, useState} from "react";
import {Container} from "../Container";
import {DevicesList} from "../../model";
import {HostSelect, OptionType} from "../common/HostSelect";
import {useCommandQuery} from "../../services/useCommandQuery";
import {useLogger} from "@wisual/logger";
import {invoke} from "@tauri-apps/api/tauri";

export function AudioIOOptionsForm() {
  const logger = useLogger("AudioIOOptionsForm");
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
    (option: OptionType<string>) => {
      const value = option.value;
      setInputDevice(value);
      invoke("set_input_device_command", {
        inputDeviceId: value,
      }).catch((err) => logger.error(err));
    },
    [setInputDevice, logger]
  );
  const onChangeOutputDevice = useCallback(
    (option: OptionType<string>) => {
      const value = option.value;
      setOutputDevice(value);
      invoke("set_output_device_command", {
        outputDeviceId: value,
      }).catch((err) => logger.error(err));
    },
    [setOutputDevice, logger]
  );
  const onChangeAudioDriver = useCallback(
    (option: OptionType<string>) => {
      const value = option.value;
      setHost(value);
      invoke("set_audio_driver_command", { hostId: value }).catch((err) =>
        logger.error(err)
      );
    },
    [setHost, logger]
  );

  return (
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
  );
}
