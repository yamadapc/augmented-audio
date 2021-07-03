export interface MessageWrapper<Message> {
  id?: string | null;
  channel: string;
  message: Message;
}

export type ServerMessage = MessageWrapper<ServerMessageInner>;
export type ServerMessageInner =
  | PublishParametersMessage
  | ParameterValueMessage;

export interface PublishParametersMessage {
  type: "PublishParameters";
  parameters: ParameterDeclarationMessage[];
}

export interface ParameterValueMessage {
  type: "ParameterValue";
  id: string;
  value: number;
}

export type ParameterType = "Number";

export interface ParameterDeclarationMessage {
  id: string;
  name: string;
  label: string;
  text: string;
  value: number;
  valueRange: [number, number];
  valueType: ParameterType;
  valuePrecision: number;
}

export type ClientMessage = MessageWrapper<ClientMessageInner>;
export type ClientMessageInner =
  | AppStartedMessage
  | SetParameterMessage
  | LogMessage;

export interface AppStartedMessage {
  type: "AppStarted";
}

export interface SetParameterMessage {
  type: "SetParameter";
  parameterId: string;
  value: number;
}

export interface LogMessage {
  type: "Log";
  level: string;
  message: string;
}
