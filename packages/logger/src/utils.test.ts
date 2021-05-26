import { shouldSkipLogging } from "./utils";

describe("shouldSkipLogging", () => {
  it("skips on TEST", () => {
    process.env.NODE_ENV = "test";
    expect(shouldSkipLogging()).toEqual(true);
  });

  it("does not skip on dev", () => {
    process.env.NODE_ENV = "development";
    expect(shouldSkipLogging()).toEqual(false);
  });
});
