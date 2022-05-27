import { Logger, } from "./logger";

import { useContext, useMemo } from "react";
import { LoggerMap } from "./logger/types";
import { LoggerContext } from "./logger-react-context/LoggerContext";
import { LoggerFactory } from "./logger-factory/LoggerFactory";

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
