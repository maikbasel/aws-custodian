import { useContext, useEffect, useState } from 'react';
import { DIContext } from '@/context/di-context';
import { Parameter } from '@/modules/parameters/core/domain';
import { getParameters } from '@/modules/parameters/application/get-parameters';
import { useCurrentProfileStore } from '@/stores/use-current-profile-store';
import { getAvailableParameters } from '@/modules/parameters/application/get-available-parameters';
import useSWR, { mutate } from 'swr';

const BATCH_SIZE = 10;

export const useParameters = () => {
  const context = useContext(DIContext);
  if (!context) {
    throw new Error('useParameters must be used inside the DIContextProvider');
  }

  const { current } = useCurrentProfileStore();

  const fetchAvailableParameters = async () => {
    const result = await getAvailableParameters(
      context.parameterDataSPI,
      current
    );
    if (result.isErr()) {
      const backendError = result.unwrapErr();
      throw new Error(`${backendError.code}: ${backendError.message}`);
    }
    return result.unwrap().names; // Return just the names
  };

  const [parameters, setParameters] = useState<Parameter[]>([]);
  const [error, setError] = useState<Error | null>(null);

  const { data: names, error: namesError } = useSWR(
    'get_available_parameters',
    () => fetchAvailableParameters(),
    { revalidateOnFocus: false }
  );

  useEffect(() => {
    mutate('get_available_parameters', undefined, {
      revalidate: true, // Trigger re-fetch
    }).then(() =>
      console.debug('Invalidating available parameters and refetching.')
    );
    setParameters([]);
  }, [current, context]);

  useEffect(() => {
    if (!names) return; // Wait until names are fetched

    const fetchParametersBatch = async (names: string[]) => {
      const fetchBatch = async (batch: string[]) => {
        const result = await getParameters(
          context.parameterDataSPI,
          current,
          batch
        );

        if (result.isErr()) {
          const backendError = result.unwrapErr();
          setError(Error(`${backendError.code}: ${backendError.message}`));
        }

        const parameters = result.unwrap().parameters;
        setParameters((prevState) => [...prevState, ...parameters]);
      };

      const totalBatches = Math.ceil(names.length / BATCH_SIZE);
      const promises = [];
      for (let i = 0; i < totalBatches; i++) {
        const start = i * BATCH_SIZE;
        const end = start + BATCH_SIZE;
        const batch = names.slice(start, end);
        promises.push(fetchBatch(batch));
      }

      await Promise.allSettled(promises); // Wait for all batches to finish
    };

    fetchParametersBatch(names).then(() =>
      console.debug('Fetching parameters', names)
    );
  }, [names, context, current]);

  return {
    parameters,
    isLoading: !names && !namesError,
    error,
  };
};
