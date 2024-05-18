import { create } from 'zustand';

interface CurrentProfileStore {
  current: string;
  setCurrent: (profile: string) => void;
}

export const useCurrentProfile = create<CurrentProfileStore>((set) => ({
  current: '',
  setCurrent: (profile) => set(() => ({ current: profile })),
}));
