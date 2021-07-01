import {DelegatingSink, LoggerFactory, LoggerSink} from "@wisual/logger";
import {LogMessage} from "@wisual/logger/lib/logger/types";

import {invoke} from "@tauri-apps/api";
import {getDefaultSink} from "@wisual/logger/lib/logger-sink";

export class TauriLoggerSink implements LoggerSink {
  log(message: LogMessage): void {
    invoke("log_command", { message }).catch((err) => {
      console.error("Failed to log", err);
    });
  }
}

export function getSink(): LoggerSink {
  const defaultSink = getDefaultSink();
  const tauriSink = new TauriLoggerSink();
  return new DelegatingSink([defaultSink, tauriSink]);
}

export function setupLogging() {
  LoggerFactory.shared.setSink(getSink());
}
