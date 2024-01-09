'use client';

import useSWR, { Fetcher } from 'swr';
import { invoke } from '@tauri-apps/api/tauri'; // see https://stackoverflow.com/a/77264549

import { ProfileSet } from '@/modules/profiles/profile';
export const useProfile = () => {
  const fetcher: Fetcher<ProfileSet, string> = (cmd: string) =>
    invoke<ProfileSet>(cmd);

  const {
    data: profileSet,
    error,
    isLoading,
  } = useSWR<ProfileSet>('get_profiles', fetcher);

  return { profileSet, error, isLoading };
};
