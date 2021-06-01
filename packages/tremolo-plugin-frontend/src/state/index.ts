import {
  ClientMessageInner,
  ParameterDeclarationMessage,
  ServerMessage,
} from "../common/protocol";
import { injectAll, singleton } from "tsyringe";
import { MessageTransport } from "@wisual/webview-transport";

export class ParametersStore {
  parameters: ParameterDeclarationMessage[] = [];
  parameterValues: { [parameterId: string]: number } = {};
}

export interface MessageHandler {
  handle(
    serverMessage: ServerMessage,
    postMessage: (msg: ClientMessageInner, id?: string) => void
  ): void;
}

@singleton()
export class ParameterMessageHandler implements MessageHandler {
  private parametersStore: ParametersStore;

  constructor(parametersStore: ParametersStore) {
    this.parametersStore = parametersStore;
  }

  handle(
    serverMessage: ServerMessage,
    postMessage: (msg: ClientMessageInner, id?: string) => void
  ): void {
    switch (serverMessage.message.type) {
      case "PublishParameters":
        this.parametersStore.parameters = serverMessage.message.parameters;
        break;
    }
  }
}

@singleton()
export class MessageHandlingService {
  private messageHandlers: MessageHandler[];
  private transport: MessageTransport<ServerMessage, ClientMessageInner>;

  constructor(
    messageHandlers: MessageHandler[],
    transport: MessageTransport<ServerMessage, ClientMessageInner>
  ) {
    this.messageHandlers = messageHandlers;
    this.transport = transport;
  }

  start() {
    this.transport.addMessageListener((msg) => {
      this.messageHandlers.forEach((handler) => {
        handler.handle(msg, (msg, id) => {
          this.transport.postMessage("default", msg, id);
        });
      });

      this.transport.postMessage("default", {
        type: "Log",
        level: "info",
        message: `ACK:${msg.message.type}`,
      });
    });
  }
}
