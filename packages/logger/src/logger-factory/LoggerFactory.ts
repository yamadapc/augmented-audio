import { Logger } from "../logger";
import { LoggerMap, LogLevel } from "../logger/types";
import { LoggerSink, getDefaultSink } from "../logger-sink";

export class LoggerFactory {
  static shared = new LoggerFactory();

  private level: LogLevel = "info";
  private sink: LoggerSink = getDefaultSink();

  setSink(sink: LoggerSink) {
    this.sink = sink;
  }

  setLevel(logLevel: LogLevel) {
    this.level = logLevel;
  }

  getLogger(
    name: string,
    context: LoggerMap = {},
    parent: Logger | null = null,
    level: LogLevel | null = null
  ): Logger {
    return new Logger({
      name,
      context,
      parent,
      getSink: () => this.sink,
      getLevel: () => level || this.level,
    });
  }

  static setSink(sink: LoggerSink) {
    LoggerFactory.shared.setSink(sink);
  }

  static setLevel(logLevel: LogLevel) {
    LoggerFactory.shared.setLevel(logLevel);
  }

  static getLogger(
    name: string,
    context: LoggerMap = {},
    parent: Logger | null = null,
    level: LogLevel | null = null
  ): Logger {
    return LoggerFactory.shared.getLogger(name, context, parent, level);
  }
}
