import { LogMessage } from "../logger/types";
import { LoggerSink } from "./LoggerSink";

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
