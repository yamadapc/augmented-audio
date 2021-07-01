export interface AudioDevice {
  name: string;
}

export interface DevicesList {
  inputDevices: AudioDevice[];
  outputDevices: AudioDevice[];
}

export interface HostState {
  pluginPath: string | null,
  audioInputFilePath: string | null,
}