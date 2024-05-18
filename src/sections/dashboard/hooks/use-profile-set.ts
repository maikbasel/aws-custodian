'use client';

import useSWR, { Fetcher } from 'swr';
import { invoke } from '@tauri-apps/api/tauri';
import { ProfileSet, profileSetSchema } from '@/modules/profiles/domain';

const fetcher: Fetcher<ProfileSet, string> = async (cmd: string) => {
  const data = await invoke<Record<string, never>>(cmd);

  return profileSetSchema.parse(data);
};

export const useProfileSet = () => {
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
