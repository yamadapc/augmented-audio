import "reflect-metadata";
import "./App.css";
import React from "react";
import HudPanel from "./HudPanel";
import Controls from "./Controls";
import { Component } from "react";
import {
  DefaultMessageTransport,
  WebkitMessageTransport,
  WebSocketsMessageTransport,
} from "@wisual/webview-transport";
import { LoggerFactory } from "@wisual/logger";
import { ClientMessageInner, ServerMessageInner } from "../common/protocol";

class App extends Component {
  private logger = LoggerFactory.getLogger("App");
  private transport!: DefaultMessageTransport<
    ServerMessageInner,
    ClientMessageInner
  >;

  componentDidMount() {
    try {
      // TODO - convert to typescript & use DI container
      this.transport = new DefaultMessageTransport(
        new WebSocketsMessageTransport(),
        new WebkitMessageTransport()
      );

      this.attachListeners();

      this.transport
        .setup()
        .then(() => {
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

  attachListeners() {
    this.transport.addMessageListener((msg) => {
      this.logger.info("Received message", msg);
    });
  }

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
