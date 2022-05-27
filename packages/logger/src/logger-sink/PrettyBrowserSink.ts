import { LogMessage } from "../logger/types";
import { getConsoleMethod, LoggerSink } from "./LoggerSink";

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
