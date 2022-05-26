import {LogLevel, LogMessage} from "../logger/types";
import chalk from "chalk";

export interface LoggerSink {
  log(message: LogMessage): void;
}

function getConsoleMethod(level: LogLevel): "log" | "debug" | "warn" | "error" {
  switch (level) {
    case "info":
      return "log";
    case "debug":
      return "debug";
    case "warn":
      return "warn";
    case "error":
      return "error";
  }
}

function getLevelColor(level: LogLevel): "cyan" | "green" | "yellow" | "red" {
  switch (level) {
    case "info":
      return "green";
    case "debug":
      return "cyan";
    case "warn":
      return "yellow";
    case "error":
      return "red";
  }
}

export class PrettyConsoleSink implements LoggerSink {
  static shared = new PrettyConsoleSink();

  log(message: LogMessage) {
    const variables = JSON.stringify({
      ...message.context,
      ...message.variables,
    });
    console[getConsoleMethod(message.level)](
      `${chalk.keyword(getLevelColor(message.level))(
        message.level.toUpperCase()
      )} [${chalk.blackBright(message.time.toISOString())}] (${message.logger}) ${chalk.white.bold(message.message)} ${variables}`
    );
  }
}

export class DelegatingSink implements LoggerSink {
  private children: LoggerSink[];

  constructor(children: LoggerSink[]) {
    this.children = children;
  }

  log(message: LogMessage) {
    this.children.forEach((child) => {
      child.log(message);
    });
  }
}

export class PrettyBrowserSink implements LoggerSink {
  static shared = new PrettyBrowserSink();

  log(message: LogMessage) {
    console[getConsoleMethod(message.level)](
      `%c${message.time.toISOString()} %c[${message.logger}] (${
        message.level
      }) %c${message.message} %c`,
      "opacity: 0.8",
      `color: ${message.logger}`,
      "font-weight: bold",
      "opacity: 0.8",
      {
        ...message.context,
        ...message.variables,
      }
    );
  }
}

export function getDefaultSink(): LoggerSink {
  return typeof window !== "undefined"
    ? PrettyBrowserSink.shared
    : PrettyConsoleSink.shared;
}
