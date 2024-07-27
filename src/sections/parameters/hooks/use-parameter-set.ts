import { useContext, useEffect } from 'react';
import { DIContext } from '@/context/di-context';
import useSWR, { Fetcher, mutate } from 'swr';
import { ParameterSet } from '@/modules/parameters/core/domain';
import { getParameters } from '@/modules/parameters/application/get-parameters';
import { useCurrentProfileStore } from '@/stores/use-current-profile-store';
import { BackendError } from '@/modules/error/error';

export const useParameterSet = () => {
  const context = useContext(DIContext);
  if (!context) {
    throw new Error(
      'useParameterSet must be used inside the DIContextProvider'
    );
  }

  const { current } = useCurrentProfileStore();

  const fetcher: Fetcher<ParameterSet, string> = async () => {
    const result = await getParameters(context.parameterDataSPI, current, 10);
    if (result.isErr()) {
      throw result.unwrapErr();
    }

    return result.unwrap();
  };

  const { data, error, isLoading } = useSWR<ParameterSet, BackendError>(
    'get_parameters',
    fetcher
  );

  useEffect(() => {
    const getParameters = async () => {
      if (current) {
        await mutate('get_parameters');
      }
    };

    getParameters().then(() =>
      console.debug('Get parameters after profile change')
    );
  }, [current]);

  return {
    parameterSet: data,
    error,
    isLoading,
  };
};
