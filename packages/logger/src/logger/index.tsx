import { useEffect } from "react";

import { shouldSkipLogging } from "../utils";
import { LoggerMap, LogLevel, logLevelValue } from "./types";
import { LoggerSink, getDefaultSink } from "../logger-sink";

interface LoggerOptions {
  name: string;
  parent: Logger | null;
  context: LoggerMap;
  getLevel?: (() => LogLevel) | null;
  getSink?: (() => LoggerSink) | null;
}

export class Logger {
  private readonly name: string;
  private readonly context: LoggerMap;
  private readonly parent: Logger | null;
  private rawGetLevel: (() => LogLevel | null) | null;
  private rawGetSink: (() => LoggerSink | null) | null;

  constructor({ name, parent, context, getLevel, getSink }: LoggerOptions) {
    this.name = name;
    this.parent = parent;
    this.context = context;
    this.rawGetLevel = getLevel || null;
    this.rawGetSink = getSink || null;
  }

  info(msg: any | string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("info")) {
      return;
    }
    const time = new Date();
    this.getSink().log({
      time,
      logger: this.name,
      message: msg,
      variables,
      context: this.context,
      level: "info",
    });
  }

  error(msg: string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("error")) {
      return;
    }
    const time = new Date();
    this.getSink().log({
      time,
      logger: this.name,
      message: msg,
      variables,
      context: this.context,
      level: "error",
    });
  }

  debug(msg: any | string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("debug")) {
      return;
    }
    const time = new Date();
    this.getSink().log({
      time,
      logger: this.name,
      message: msg,
      variables,
      context: this.context,
      level: "debug",
    });
  }

  warn(msg: any | string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("warn")) {
      return;
    }
    const time = new Date();
    this.getSink().log({
      time,
      logger: this.name,
      message: msg,
      variables,
      context: this.context,
      level: "warn",
    });
  }

  child(name: string, context: LoggerMap): Logger {
    return new Logger({
      name: `${this.name}>${name}`,
      parent: this,
      context: { ...this.context, ...context },
    });
  }

  onInfo(msg: string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("info")) {
      return;
    }
    useEffect(() => {
      this.info(msg, variables);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, Object.values(variables));
  }

  onError(msg: string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("error")) {
      return;
    }
    useEffect(() => {
      this.error(msg, variables);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, Object.values(variables));
  }

  onWarn(msg: string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("warn")) {
      return;
    }
    useEffect(() => {
      this.warn(msg, variables);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, Object.values(variables));
  }

  onDebug(msg: string, variables: LoggerMap = {}) {
    if (shouldSkipLogging() || !this.shouldLog("debug")) {
      return;
    }
    useEffect(() => {
      this.debug(msg, variables);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, Object.values(variables));
  }

  setLevel(logLevel: LogLevel) {
    this.rawGetLevel = () => logLevel;
  }

  getLevel(): LogLevel {
    return (
      (this.rawGetLevel && this.rawGetLevel()) ??
      this.parent?.getLevel() ??
      "info"
    );
  }

  getSink(): LoggerSink {
    return (
      (this.rawGetSink && this.rawGetSink()) ??
      this.parent?.getSink() ??
      getDefaultSink()
    );
  }

  private shouldLog(level: LogLevel): boolean {
    const loggerLevel = this.getLevel();
    return logLevelValue[level] >= logLevelValue[loggerLevel];
  }
}
