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
} from "../common/protocol";
import { container } from "tsyringe";
import { MessageHandlingService } from "../state";

interface State {
  parameters: ParameterDeclarationMessage[];
}

class App extends Component<{}, State> {
  private logger = LoggerFactory.getLogger("App");
  private transport!: DefaultMessageTransport<
    ServerMessage,
    ClientMessageInner
  >;
  private handlerService!: MessageHandlingService;
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
          this.handlerService.start();
          this.transport.postMessage("default", {
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

  // attachListeners() {
  //   this.transport.addMessageListener((msg) => {
  //     this.logger.info("Received message", msg);
  //     switch (msg.message.type) {
  //       case "PublishParameters": {
  //         this.handlePublishParameters(msg.message);
  //         break;
  //       }
  //     }
  //
  //   });
  // }

  // private handlePublishParameters(msg: PublishParametersMessage) {
  //   this.setState({
  //     parameters: msg.parameters,
  //   });
  // }

  render() {
    return (
      <div className="App">
        <HudPanel />
        <Controls />
      </div>
    );
  }
}

export default App;
