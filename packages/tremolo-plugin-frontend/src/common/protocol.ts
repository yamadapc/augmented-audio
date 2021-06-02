export interface MessageWrapper<Message> {
    id?: string | null,
    channel: string,
    message: Message,
}

export type ServerMessage = MessageWrapper<ServerMessageInner>;
export type ServerMessageInner = PublishParametersMessage;

export interface PublishParametersMessage {
    type: 'PublishParameters',
    parameters: ParameterDeclarationMessage[]
}

export type ParameterType = 'Number';

export interface ParameterDeclarationMessage {
    id: string,
    name: string,
    label: string,
    text: string,
    value: number,
    valueRange: [number, number],
    valueType: ParameterType,
    valuePrecision: number,
}

export type ClientMessage = MessageWrapper<ClientMessageInner>;
export type ClientMessageInner =
    AppStartedMessage | SetParameterMessage | LogMessage;

interface AppStartedMessage {
    type: 'AppStarted',
}

interface SetParameterMessage {
    type: 'SetParameter',
    parameterId: string,
    value: number
}

interface LogMessage {
    type: 'Log',
    level : string,
    message: string
}
