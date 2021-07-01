import {useCallback, useEffect, useState} from "react";
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";
import {useLogger} from "@wisual/logger";

interface CommandState<T> {
  loading: boolean;
  data: null | T;
  error: null | Error;
}

interface CommandResult<T> {
  loading: boolean;
  data: null | T;
  error: null | Error;
  reload: () => void;
}

interface UseCommandParams {
  skip: boolean;
  args: InvokeArgs;
}

export function useCommandQuery<T>(
  command: string,
  params?: UseCommandParams
): CommandResult<T> {
  const logger = useLogger(`CommandQuery::${command}`);
  const [state, setState] = useState<CommandState<T>>({
    loading: !params?.skip,
    data: null,
    error: null,
  });

  const reload = useCallback(() => {
    if (params?.skip) {
      return;
    }

    const run = async () => {
      setState((state) =>
        state.loading ? state : { ...state, loading: true }
      );
      logger.info("Running command");
      const result = await invoke<T>(command, params?.args);
      logger.info("Got command response", { result });

      setState({ error: null, data: result, loading: false });
    };

    run().catch((err) => {
      logger.error("Failed running command", { error: err });
      setState((state) => ({ error: err, data: state.data, loading: false }));
    });
  }, [command, params?.args, params?.skip, logger]);

  useEffect(() => {
    reload();
  }, [reload]);

  return {
    ...state,
    reload,
  };
}
