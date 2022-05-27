import {LoggerSink} from "./LoggerSink";
import { PrettyBrowserSink } from "./PrettyBrowserSink";
import { PrettyConsoleSink } from "./PrettyConsoleSink";

export { DelegatingSink } from './DelegatingSink';
export { PrettyBrowserSink, PrettyConsoleSink, }
export type { LoggerSink }

export function getDefaultSink(): LoggerSink {
  return typeof window !== "undefined"
    ? PrettyBrowserSink.shared
    : PrettyConsoleSink.shared;
}
