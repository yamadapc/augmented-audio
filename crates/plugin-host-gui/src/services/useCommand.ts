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

export function useCommand<T>(
    command: string,
    params?: UseCommandParams
): CommandState<T> {
    const [loading, setLoading] = useState(!params?.skip);
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<Error | null>(null);

    useEffect(() => {
        const run = async () => {
          setLoading(true);
          const result = await invoke<T>(command, params?.args);
          setData(result);
        };

        run().catch((err) => {
            setError(err);
        });
    }, [command, params?.args]);

    return {loading, data, error};
}