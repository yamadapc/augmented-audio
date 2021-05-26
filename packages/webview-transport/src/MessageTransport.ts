import { EventEmitter } from "events";
import { InputMessageWrapper, OutgoingMessage } from "../protocol";

export abstract class MessageTransport {
  protected emitter = new EventEmitter();

  abstract setup(): void;
  abstract postMessage(
    channel: string,
    message: OutgoingMessage,
    id?: string
  ): Promise<void>;
  abstract getId(): string;

  addMessageListener(fn: (msg: InputMessageWrapper) => void): void {
    this.emitter.addListener("message", fn);
  }

  removeMessageListener(fn: (msg: InputMessageWrapper) => void): void {
    this.emitter.removeListener("message", fn);
  }
}
