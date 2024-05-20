import { create } from 'zustand';

interface ProfileFormValue {
  name: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
  outputFormat: string;
}

interface ProfileFormStore {
  formData: ProfileFormValue;
  setFormData: (value: ProfileFormValue) => void;
  resetFormData: () => void;
}

export const userProfileFormStore = create<ProfileFormStore>((set) => ({
  formData: {
    name: '',
    accessKeyId: '',
    secretAccessKey: '',
    region: '',
    outputFormat: '',
  },
  setFormData: (value: ProfileFormValue) => set(() => ({ formData: value })),
  resetFormData: () =>
    set(() => ({
      formData: {
        name: '',
        accessKeyId: '',
        secretAccessKey: '',
        region: '',
        outputFormat: '',
      },
    })),
}));
