import {LogMessage, LogLevel} from "../logger/types";

export interface LoggerSink {
    log(message: LogMessage): void;
}

export function getConsoleMethod(level: LogLevel): "log" | "debug" | "warn" | "error" {
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

export function getLevelColor(level: LogLevel): "cyan" | "green" | "yellow" | "red" {
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
