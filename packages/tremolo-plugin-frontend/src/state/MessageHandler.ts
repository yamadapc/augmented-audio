import { ClientMessageInner, ServerMessage } from "../common/protocol";

export interface MessageHandler {
  handle(
    serverMessage: ServerMessage,
    postMessage: (msg: ClientMessageInner, id?: string) => void
  ): void;
}
