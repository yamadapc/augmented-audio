import { render } from "@testing-library/react";
import { container } from "tsyringe";
import { DefaultMessageTransport } from "@wisual/webview-transport";
import App from "./App";
import { ClientMessageInner, ServerMessage } from "../message-handling/protocol";

jest.mock("./HudPanel", () => () => null);
jest.mock("./Controls", () => () => null);

test("DefaultMessageTransport can be resolved", () => {
  container.resolve<DefaultMessageTransport<ServerMessage, ClientMessageInner>>(
    DefaultMessageTransport
  );
});

test("renders without throwing", () => {
  render(<App />);
});
