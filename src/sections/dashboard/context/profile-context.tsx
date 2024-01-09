'use client';

import React, {
  createContext,
  PropsWithChildren,
  useContext,
  useMemo,
} from 'react';
import useSWR, { Fetcher } from 'swr';
// see https://stackoverflow.com/a/77264549
import { invoke } from '@tauri-apps/api/tauri';

type ProfileContextType = {
  data: Record<string, unknown> | undefined;
  error: Error | undefined;
  isLoading: boolean;
};

export const ProfileContext = createContext<ProfileContextType>({
  data: undefined,
  error: undefined,
  isLoading: true,
});

export const ProfileProvider = ({ children }: PropsWithChildren) => {
  const fetcher: Fetcher<Record<string, never>, string> = (cmd: string) =>
    invoke<Record<string, never>>(cmd);
  const { data, error, isLoading } = useSWR<Record<string, never>, Error>(
    'get_profiles',
    fetcher
  );

  const value = useMemo(
    () => ({ data, error, isLoading }),
    [data, error, isLoading]
  );
  return (
    <ProfileContext.Provider value={value}>{children}</ProfileContext.Provider>
  );
};

export const useProfileContext = () => {
  const context = useContext(ProfileContext);

  if (!context) {
    throw new Error('useThemeContext must be used inside the ThemeProvider');
  }

  return context;
};
