'use client';

import useSWR, { Fetcher } from 'swr';
import { ProfileSet } from '@/modules/profiles/core/domain';
import { useContext } from 'react';
import { DIContext } from '@/context/di-context';
import { getProfiles } from '@/modules/profiles/application/get-profiles';

export const useProfileSet = () => {
  const context = useContext(DIContext);
  if (!context) {
    throw new Error('useProfileSet must be used inside the DIContextProvider');
  }

  const fetcher: Fetcher<ProfileSet, string> = async () => {
    const result = await getProfiles(context.profileDataSPI);
    if (result.isErr()) {
      throw result.unwrapErr();
    }

    return result.unwrap();
  };

  const { data, error, isLoading } = useSWR<ProfileSet, Error>(
    'get_profiles',
    fetcher
  );

  return {
    profileSet: data,
    error,
    isLoading,
  };
};
