import "./App.css";
import React, { Component } from "react";
import HudPanel from "./HudPanel";
import Controls from "./Controls";
import { DefaultMessageTransport } from "@wisual/webview-transport";
import { LoggerFactory } from "@wisual/logger";
import {
  ClientMessageInner,
  ParameterDeclarationMessage,
  ServerMessage,
} from "@ruas/generic-parameters-editor-runtime/lib/protocol";
import { container } from "tsyringe";
import { MessageHandlingService } from "@ruas/generic-parameters-editor-runtime/lib/MessageHandlingService";
import { ParametersStore } from "@ruas/generic-parameters-editor-runtime/lib/ParametersStore";

interface State {
  parameters: ParameterDeclarationMessage[];
}

class App extends Component<{}, State> {
  private logger = LoggerFactory.getLogger("App");
  private transport?: DefaultMessageTransport<
    ServerMessage,
    ClientMessageInner
  >;
  private handlerService?: MessageHandlingService;
  private parametersStore: ParametersStore = container.resolve(ParametersStore);

  state: State = { parameters: [] };

  componentDidMount() {
    try {
      this.transport = container.resolve<
        DefaultMessageTransport<ServerMessage, ClientMessageInner>
      >(DefaultMessageTransport);
      this.handlerService = container.resolve<MessageHandlingService>(
        MessageHandlingService
      );

      this.transport
        .setup()
        .then(() => {
          this.logger.info("Transport connected");
          this.handlerService?.start();

          this.transport?.postMessage("default", {
            type: "AppStarted",
          });
        })
        .catch((err) => {
          this.logger.error(err);
        });
    } catch (err) {
      this.logger.error(err);
    }
  }

  componentWillUnmount() {
    this.handlerService?.stop();
  }

  setParameter = (id: string, value: number) => {
    this.parametersStore.setParameterValue(id, value);
    this.transport?.postMessage("default", {
      type: "SetParameter",
      parameterId: id,
      value,
    });
  };

  render() {
    return (
      <div className="App">
        <HudPanel parametersStore={this.parametersStore} />
        <Controls
          parametersStore={this.parametersStore}
          setParameter={this.setParameter}
        />
      </div>
    );
  }
}

export default App;
