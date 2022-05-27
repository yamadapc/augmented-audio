import chalk from "chalk";
import { LogMessage } from "../logger/types";
import {getConsoleMethod, getLevelColor, LoggerSink } from "./LoggerSink";

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
