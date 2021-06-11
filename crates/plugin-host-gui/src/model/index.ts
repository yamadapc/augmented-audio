interface AudioDevice {
  name: string;
}

export interface DevicesList {
  inputDevices: AudioDevice[];
  outputDevices: AudioDevice[];
}
