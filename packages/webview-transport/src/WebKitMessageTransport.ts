import { singleton } from "tsyringe";
import { MessageTransport } from "./MessageTransport";
import { LoggerFactory } from "@wisual/logger";

// @ts-ignore
const webkit: any = global.webkit;

@singleton()
export class WebkitMessageTransport<IncomingMessage, OutgoingMessage> extends MessageTransport<IncomingMessage, OutgoingMessage> {
  private logger = LoggerFactory.getLogger("WebkitMessageTransport");

  getId(): string {
    return "WebkitMessageTransport";
  }

  setup(): void {
    if (!webkit) {
      this.logger.warn("Not in WebKit view, transport will drop messages.");
    }
    // @ts-ignore
    global.__onMessage = this.onMessage;
  }

  isAvailable(): boolean {
    return !!webkit;
  }

  postMessage(channel: string, message: OutgoingMessage, id?: string): Promise<void> {
    if (webkit) {
      webkit.messageHandlers[channel].postMessage({
        ...message,
        id,
      });
    }
    return Promise.resolve();
  }

  onMessage = (msg: IncomingMessage) => {
    this.emitter.emit("message", msg);
  };
}
