import {useEffect, useState} from "react";
import {invoke, InvokeArgs} from "@tauri-apps/api/tauri";

interface CommandState<T> {
  loading: boolean;
  data: null | T;
  error: null | Error;
}

interface UseCommandParams {
  skip: boolean;
  args: InvokeArgs;
}

export function useCommandQuery<T>(
  command: string,
  params?: UseCommandParams
): CommandState<T> {
  const [state, setState] = useState<CommandState<T>>({
    loading: !params?.skip,
    data: null,
    error: null,
  });

  useEffect(() => {
    if (params?.skip) {
      return;
    }

    const run = async () => {
      setState((state) =>
        state.loading ? state : { ...state, loading: true }
      );
      const result = await invoke<T>(command, params?.args);

      setState({ error: null, data: result, loading: false });
    };

    run().catch((err) => {
      setState((state) => ({ error: err, data: state.data, loading: false }));
    });
  }, [command, params?.args, params?.skip]);

  return state;
}
