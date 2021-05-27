import 'reflect-metadata';
import "./App.css";
import HudPanel from "./HudPanel";
import Controls from "./Controls";
import { Component } from "react";
import {
  DefaultMessageTransport,
  WebkitMessageTransport,
  WebSocketsMessageTransport,
} from "@wisual/webview-transport";
import { LoggerFactory } from "@wisual/logger";

class App extends Component {
  logger = LoggerFactory.getLogger("App");

  componentDidMount() {
    try {
      // TODO - convert to typescript & use DI container
      this.transport = new DefaultMessageTransport(
        new WebSocketsMessageTransport(),
        new WebkitMessageTransport()
      );

      this.attachListeners(this.transport);

      this.transport.setup();
      this.transport.postMessage("default", {
        type: "AppStarted",
      });
    } catch (err) {
      this.logger.error(err);
    }
  }

  attachListeners(transport) {
    transport.addMessageListener((msg) => {
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
