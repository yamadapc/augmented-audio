import { inject, singleton } from "tsyringe";
import { MessageTransport } from "./MessageTransport";
import { LoggerFactory } from "@wisual/logger";
import { WebSocketsMessageTransport } from "./WebSocketsMessageTransport";
import { WebkitMessageTransport } from "./WebKitMessageTransport";
import { OutgoingMessage } from "../protocol";

/**
 * MessageTransport that forwards messages onto WebSockets or WebKit depending on WebKit being available.
 */
@singleton()
export class DefaultMessageTransport extends MessageTransport {
  private transport: MessageTransport;
  private logger = LoggerFactory.getLogger("DefaultMessageTransport");

  getId(): string {
    return `DefaultMessageTransport@${this.transport.getId()}`;
  }

  constructor(
    @inject(WebSocketsMessageTransport)
    webSocketsMessageTransport: WebSocketsMessageTransport,
    @inject(WebkitMessageTransport)
    webKitMessageTransport: WebkitMessageTransport
  ) {
    super();
    this.transport = webKitMessageTransport.isAvailable()
      ? webKitMessageTransport
      : webSocketsMessageTransport;
  }

  setup() {
    this.transport.setup();
    this.transport.addMessageListener((msg) => {
      this.emitter.emit("message", msg);
    });
  }

  postMessage(
    channel: string,
    message: OutgoingMessage,
    id?: string
  ): Promise<void> {
    return this.transport.postMessage(channel, message, id);
  }
}
