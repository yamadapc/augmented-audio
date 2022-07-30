import {Logger} from "./logger";
import {
  LoggerContext,
  wrapWithLogger,
} from "./logger-react-context/LoggerContext";
import {LoggerFactory} from "./logger-factory/LoggerFactory";
import {LoggerMap} from "./logger/types";
import {
  DelegatingSink,
  LoggerSink,
  PrettyBrowserSink,
  PrettyConsoleSink
} from "./logger-sink";

export type { LoggerMap, LoggerSink };
export {
  Logger,
  LoggerContext,
  LoggerFactory,
  PrettyConsoleSink,
  PrettyBrowserSink,
  DelegatingSink,
};
export { useLogger } from './useLogger';

export { wrapWithLogger };
