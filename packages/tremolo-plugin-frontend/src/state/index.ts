import { ClientMessageInner, ServerMessage } from "../common/protocol";
import { injectAll, singleton } from "tsyringe";
import {
  DefaultMessageTransport,
  MessageTransport,
} from "@wisual/webview-transport";
import { MessageHandler } from "./MessageHandler";
import "./MessageHandlerRegistry";

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
    this.transport.addMessageListener(this.onMessage);
  }

  stop() {
    this.transport.removeMessageListener(this.onMessage);
  }

  onMessage = (msg: ServerMessage) => {
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
  };
}
