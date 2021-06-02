import { render } from "@testing-library/react";
import App from "./App";

jest.mock("./HudPanel", () => () => null);
jest.mock("./Controls", () => () => null);

test("renders without throwing", () => {
  render(<App />);
});
