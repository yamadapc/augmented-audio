import {LogLevel, LogMessage} from "../logger/types";
import memoize from "lodash/memoize";
import chroma from "chroma-js";
import chalk from "chalk";

export interface LoggerSink {
  log(message: LogMessage): void;
}

const getNameColor = memoize((_name: string, light) =>
  light ? chroma.random().brighten(1.5).hex() : chroma.random().darken(1).hex()
);

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
      )} [${chalk.blackBright(message.time.toISOString())}] (${chalk.hex(
        getNameColor(message.logger, true)
      )(message.logger)}) ${chalk.white.bold(message.message)} ${variables}`
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

// @ts-ignore
const useLightColors =
  typeof localStorage != "undefined" &&
  localStorage.getItem("wisual:use-light-colors-logger") === "true";

export class PrettyBrowserSink implements LoggerSink {
  static shared = new PrettyBrowserSink();

  log(message: LogMessage) {
    console[getConsoleMethod(message.level)](
      `%c${message.time.toISOString()} %c[${message.logger}] (${
        message.level
      }) %c${message.message} %c`,
      "opacity: 0.8",
      `color: ${getNameColor(message.logger, useLightColors)}`,
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
