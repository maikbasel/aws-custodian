'use client';

import { useContext } from 'react';
import { DIContext } from '@/context/di-context';
import { Profile } from '@/modules/profiles/core/domain';
import { createProfile } from '@/modules/profiles/application/create-profile';
import { deleteProfile } from '@/modules/profiles/application/delete-profile';
import { deleteProfiles } from '@/modules/profiles/application/delete-profiles';
import { editProfile } from '@/modules/profiles/application/edit-profile';
import { validateCredentials } from '@/modules/credentials/application/validate-credentials';

export const useProfileForm = () => {
  const context = useContext(DIContext);
  if (!context) {
    throw new Error('useProfileForm must be used inside the DIContextProvider');
  }

  return {
    createProfile: async (profile: Profile) =>
      createProfile(context.profileDataSPI, profile),
    editProfile: async (profile: Profile) =>
      editProfile(context.profileDataSPI, profile),
    deleteProfile: async (profileName: string) =>
      deleteProfile(context.profileDataSPI, profileName),
    deleteProfiles: async (profileNames: string[]) =>
      deleteProfiles(context.profileDataSPI, profileNames),
    validateCredentials: async (profileName: string) =>
      validateCredentials(context.credentialsDataSPI, profileName),
  };
};
