import React, { ReactElement, ReactNode } from "react";

import { Logger } from "../logger";
import {LoggerFactory} from "../logger-factory/LoggerFactory";

export const LoggerContext = React.createContext<Logger>(
  LoggerFactory.getLogger("app")
);

/**
 * Wraps a sub-tree with a logger so that new loggers in the sub-tree will use it as their parent.
 *
 * @param logger
 * @param node
 */
export function wrapWithLogger(logger: Logger, node: ReactNode): ReactElement {
  return <LoggerContext.Provider value={logger}>{node}</LoggerContext.Provider>;
}
