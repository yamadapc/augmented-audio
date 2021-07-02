import {listen} from "@tauri-apps/api/event";
import {useEffect, useState} from "react";
import {useLogger} from "@wisual/logger";

export function useTauriEvent<T>(eventName: string) {
  const logger = useLogger(`TauriEventListener::${eventName}`);
  const [state, setState] = useState<T | null>(null);

  useEffect(() => {
    const subscriptionPromise = listen<T>(eventName, (data) => {
      logger.info("Received event data", { eventName, data });
      setState(data.payload);
    });

    return () => {
      subscriptionPromise
        .then((unsubscribe) => {
          unsubscribe();
        })
        .catch((err) => {
          logger.error(`Failed to subscribe to ${eventName}`, err);
        });
    };
  }, [setState, eventName]);

  return state;
}
