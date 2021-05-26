import { Logger } from "./index";
import { shouldSkipLogging } from "../utils";
import { getDefaultSink, LoggerSink } from "../logger-sink";

jest.mock("../utils");

describe("Logger", () => {
  let logger = new Logger({
    name: "test",
    parent: null,
    context: {},
  });

  beforeEach(() => {
    // @ts-ignore
    shouldSkipLogging.mockReturnValue(false);
    jest.spyOn(console, "log").mockImplementation(() => {});
    jest.spyOn(console, "error").mockImplementation(() => {});
    jest.spyOn(console, "warn").mockImplementation(() => {});
    jest.spyOn(console, "debug").mockImplementation(() => {});
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  it("forwards calls to console", () => {
    logger.setLevel("debug");
    logger.info("here");
    expect(console.log).toHaveBeenCalledTimes(1);
    logger.error("here");
    expect(console.error).toHaveBeenCalledTimes(1);
    logger.warn("here");
    expect(console.warn).toHaveBeenCalledTimes(1);
    logger.debug("here");
    expect(console.debug).toHaveBeenCalledTimes(1);
  });

  describe("getLevel", () => {
    it("goes up the tree to figure level", () => {
      const logger = new Logger({
        name: "Root",
        context: {},
        parent: null,
        getLevel: () => "warn",
      });

      const child = logger.child("child", {});
      expect(child.getLevel()).toEqual("warn");
    });

    it("defaults to info", () => {
      const logger = new Logger({
        name: "Root",
        context: {},
        parent: null,
        getLevel: null,
      });

      const child = logger.child("child", {});
      expect(child.getLevel()).toEqual("info");
    });
  });

  describe("getSink", () => {
    it("goes up the tree to figure sink", () => {
      // @ts-ignore
      const mockSink: LoggerSink = {};
      const logger = new Logger({
        name: "Root",
        context: {},
        parent: null,
        getSink: () => mockSink,
      });

      const child = logger.child("child", {});
      expect(child.getSink()).toEqual(mockSink);
    });

    it("defaults to default", () => {
      const logger = new Logger({
        name: "Root",
        context: {},
        parent: null,
        getSink: null,
      });

      const child = logger.child("child", {});
      expect(child.getSink()).toEqual(getDefaultSink());
    });
  });
});
