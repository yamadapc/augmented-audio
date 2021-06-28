import {invoke} from "@tauri-apps/api/tauri";
import {useCallback} from "react";
import {useLogger} from "@wisual/logger";

export function useCommandInvoke(command: string) {
  const logger = useLogger(`use-command:${command}`);
  const callback = useCallback(() => {
    const promise = invoke(command);
    logger.info(`Calling ${command}...`, { command });

    promise
      .then(() => {
        logger.info(`${command} finished`, { command });
      })
      .catch((err) => {
        logger.error(`${command} failed`, { command, err });
      });

    return promise;
  }, [logger, command]);

  return callback;
}
