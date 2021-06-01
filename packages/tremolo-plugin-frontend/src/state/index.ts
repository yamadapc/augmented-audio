import {
  ClientMessageInner,
  ParameterDeclarationMessage,
  ServerMessage,
} from "../common/protocol";
import { injectAll, registry, singleton } from "tsyringe";
import {
  MessageTransport,
  DefaultMessageTransport,
} from "@wisual/webview-transport";
import { LoggerFactory } from "@wisual/logger";

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
  private logger = LoggerFactory.getLogger("ParameterMessageHandler");

  constructor(parametersStore: ParametersStore) {
    this.parametersStore = parametersStore;
  }

  handle(
    serverMessage: ServerMessage,
    postMessage: (msg: ClientMessageInner, id?: string) => void
  ): void {
    switch (serverMessage.message.type) {
      case "PublishParameters":
        this.logger.info("Got parameters message");
        this.parametersStore.parameters = serverMessage.message.parameters;
        break;
    }
  }
}

@registry([{ token: "MessageHandler", useToken: ParameterMessageHandler }])
export class MessageHandlerRegistry {}

@singleton()
export class MessageHandlingService {
  private messageHandlers: MessageHandler[];
  private transport: MessageTransport<ServerMessage, ClientMessageInner>;

  constructor(
    @injectAll("MessageHandler")
    messageHandlers: MessageHandler[],
    transport: DefaultMessageTransport<ServerMessage, ClientMessageInner>
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
