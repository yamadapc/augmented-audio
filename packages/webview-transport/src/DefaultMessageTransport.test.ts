import "reflect-metadata";
import { DefaultMessageTransport } from "./DefaultMessageTransport";
import { container } from "tsyringe";

describe("DefaultMessageTransport", () => {
  it("can be default resolved", () => {
    container.resolve(DefaultMessageTransport);
  });
});
