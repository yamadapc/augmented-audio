import {useContext, useMemo} from "react";

import {Logger} from "./logger";
import {LoggerContext, wrapWithLogger,} from "./logger-react-context/LoggerContext";
import {LoggerFactory} from "./logger-factory/LoggerFactory";
import {LoggerMap} from "./logger/types";
import {DelegatingSink, LoggerSink, PrettyBrowserSink, PrettyConsoleSink} from "./logger-sink";

export type { LoggerMap, LoggerSink };
export {
  Logger,
  LoggerContext,
  LoggerFactory,
  PrettyConsoleSink,
  PrettyBrowserSink,
  DelegatingSink,
};

/**
 * Creates a logger with `name` that inherits its data context from react context.
 *
 * @param name
 * @param context
 * @param parent
 */
export function useLogger(name: string, context?: LoggerMap, parent?: Logger) {
  const contextLogger: Logger = useContext(LoggerContext);

  return useMemo(() => {
    if (parent || contextLogger) {
      const p = parent || contextLogger;
      return p.child(name, context || {});
    }

    return LoggerFactory.getLogger(name, context);
  }, [name, context, parent, contextLogger]);
}

export { wrapWithLogger };
