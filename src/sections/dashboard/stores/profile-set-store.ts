import { create } from 'zustand';
import { ProfileSet } from '@/modules/profiles/domain';

interface ProfileSetStore {
  profileSet: ProfileSet;
  setProfileSet: (profiles: ProfileSet) => void;
  error?: Error;
  setError: (error: Error) => void;
}

export const useProfileSetStore = create<ProfileSetStore>((set) => ({
  profileSet: { profiles: [] },
  setProfileSet: (profileSet) => set(() => ({ profileSet: profileSet })),
  error: undefined,
  setError: (error) => set(() => ({ error: error })),
}));
