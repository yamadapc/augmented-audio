import { inject, singleton } from "tsyringe";
import { MessageTransport } from "./MessageTransport";
import { WebSocketsMessageTransport } from "./WebSocketsMessageTransport";
import { WebkitMessageTransport } from "./WebKitMessageTransport";

/**
 * MessageTransport that forwards messages onto WebSockets or WebKit depending on WebKit being available.
 */
@singleton()
export class DefaultMessageTransport<IncomingMessage, OutgoingMessage> extends MessageTransport<IncomingMessage, OutgoingMessage> {
  private transport: MessageTransport<IncomingMessage, OutgoingMessage>;

  getId(): string {
    return `DefaultMessageTransport@${this.transport.getId()}`;
  }

  constructor(
    @inject(WebSocketsMessageTransport)
    webSocketsMessageTransport: WebSocketsMessageTransport<IncomingMessage, OutgoingMessage>,
    @inject(WebkitMessageTransport)
    webKitMessageTransport: WebkitMessageTransport<IncomingMessage, OutgoingMessage>
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
