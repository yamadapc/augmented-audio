import { EventEmitter } from "events";

export abstract class MessageTransport<IncomingMessage, OutgoingMessage> {
  protected emitter = new EventEmitter();

  abstract setup(): Promise<void>;
  abstract postMessage(
    channel: string,
    message: OutgoingMessage,
    id?: string
  ): Promise<void>;
  abstract getId(): string;

  addMessageListener(fn: (msg: IncomingMessage) => void): void {
    this.emitter.addListener("message", fn);
  }

  removeMessageListener(fn: (msg: IncomingMessage) => void): void {
    this.emitter.removeListener("message", fn);
  }
}
