import { registry } from "tsyringe";
import { WebSocketsMessageTransport } from "./WebSocketsMessageTransport";
import { WebkitMessageTransport } from "./WebKitMessageTransport";

@registry([
  { token: "MessageTransport", useToken: WebSocketsMessageTransport },
  { token: "MessageTransport", useToken: WebkitMessageTransport },
])
export class MessageTransportRegistry {}
