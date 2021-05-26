import { singleton } from "tsyringe";
import WebSocket from "isomorphic-ws";
import { MessageTransport } from "./MessageTransport";
import { LoggerFactory } from "@wisual/logger";
import { delay } from "./util";
import { OutgoingMessage } from "../protocol";

@singleton()
export class WebSocketsMessageTransport extends MessageTransport {
  private connection: null | WebSocket = null;
  private logger = LoggerFactory.getLogger("WebSocketsMessageTransport");
  private connectedPromise: Promise<void> | null = null;

  getId(): string {
    return "WebSocketsMessageTransport";
  }

  setup() {
    this.logger.info("Opening WS connection");
    return this.makeConnection();
  }

  private makeConnection() {
    if (this.connection) {
      this.connection.onclose = null;
      this.connection.onmessage = null;
      this.connection.onopen = null;
    }

    this.connection = new WebSocket("ws://localhost:9510/ws");
    this.connectedPromise = new Promise((resolve) => {
      let hasOpened = false;
      if (this.connection) {
        this.connection.onopen = () => {
          hasOpened = true;
          resolve();
        };

        this.connection.onmessage = (msg: any) => {
          this.onMessage(msg);
        };

        this.connection.onerror = (err: any) => {
          this.onError(err);
        };

        this.connection.onclose = async () => {
          await this.onCloseConnection();
          if (!hasOpened) {
            hasOpened = true;
            resolve();
          }
        };
      }
    });
    return this.connectedPromise;
  }

  private onMessage(msg: MessageEvent) {
    this.logger.debug("WS message", { data: msg.data });
    const data = JSON.parse(msg.data);
    this.emitter.emit("message", data);
  }

  async postMessage(
    channel: string,
    message: OutgoingMessage,
    id?: string
  ): Promise<void> {
    await this.connectedPromise;

    if (!this.connection) {
      this.logger.error("Trying to send message before connecting");
      return;
    }

    this.connection.send(JSON.stringify({ channel, message, id }));
  }

  private async onCloseConnection() {
    this.logger.warn("Connection has been closed, re-connecting after timeout");
    await delay(1000);
    return this.makeConnection();
  }

  private onError(err: Event) {
    this.logger.error("Error on ws connection", { error: err });
  }
}
