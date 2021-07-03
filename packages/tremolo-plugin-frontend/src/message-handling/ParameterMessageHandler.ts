import { LoggerFactory } from "@wisual/logger";
import { singleton } from "tsyringe";
import { MessageHandler } from "./MessageHandler";
import { ParametersStore } from "./ParametersStore";
import { ClientMessageInner, ServerMessage } from "./protocol";

@singleton()
export class ParameterMessageHandler implements MessageHandler {
  private parametersStore: ParametersStore;
  private logger = LoggerFactory.getLogger("ParameterMessageHandler");

  constructor(parametersStore: ParametersStore) {
    this.parametersStore = parametersStore;
  }

  handle(
    serverMessage: ServerMessage,
    postMessage: (msg: ClientMessageInner, id?: string) => void
  ): void {
    switch (serverMessage.message.type) {
      case "PublishParameters":
        this.logger.info("Got parameters message");
        this.parametersStore.setParameters(serverMessage.message.parameters);
        break;
      case "ParameterValue":
        this.logger.info("Got parameters message");
        this.parametersStore.setParameterValue(
          serverMessage.message.id,
          serverMessage.message.value
        );
        break;
    }
  }
}
