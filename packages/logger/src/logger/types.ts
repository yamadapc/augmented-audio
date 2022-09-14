export interface LoggerMap {
  [key: string]: any;
}

export type LogLevel = "info" | "debug" | "warn" | "error";

export const logLevelValue: {
  [level: string]: number;
} = {
  error: 3,
  warn: 2,
  info: 1,
  debug: 0,
};

export interface LogMessage {
  level: LogLevel;
  message: unknown;
  time: Date;
  logger: string;
  variables: LoggerMap;
  context: LoggerMap;
}
