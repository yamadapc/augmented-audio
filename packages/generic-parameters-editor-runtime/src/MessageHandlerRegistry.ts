import { registry } from "tsyringe";
import { ParameterMessageHandler } from "./ParameterMessageHandler";

@registry([{ token: "MessageHandler", useToken: ParameterMessageHandler }])
export class MessageHandlerRegistry {}
