import { create } from 'zustand';

interface ProfileStore {
  current: string;
  setCurrent: (profile: string) => void;
}

export const useProfile = create<ProfileStore>((set) => ({
  current: '',
  setCurrent: (profile) => set(() => ({ current: profile })),
}));
