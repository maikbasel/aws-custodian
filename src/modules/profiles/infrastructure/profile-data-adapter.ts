import {
  Profile,
  ProfileDataError,
  ProfileSet,
  profileSetSchema,
  ProfileDataSPI,
} from '@/modules/profiles/core/domain';
import { Err, Ok, Result } from 'oxide.ts';
import { invoke } from '@tauri-apps/api/tauri';

export function createProfileDataAdapter(): ProfileDataSPI {
  return {
    loadProfiles,
    saveProfile,
    updateProfile,
    removeProfile,
    removeProfiles,
  };
}

async function loadProfiles(): Promise<Result<ProfileSet, ProfileDataError>> {
  return invoke<Record<string, never>>('get_profiles')
    .then((data) => Ok(profileSetSchema.parse(data)))
    .catch((err) => Err(new ProfileDataError(err.message)));
}

async function saveProfile(
  profile: Profile
): Promise<Result<void, ProfileDataError>> {
  return invoke<void>('create_profile', { profile })
    .then(() => Ok(undefined))
    .catch((err) => Err(new ProfileDataError(err.message)));
}

async function updateProfile(
  profile: Profile
): Promise<Result<void, ProfileDataError>> {
  return invoke<void>('edit_profile', { profile })
    .then(() => Ok(undefined))
    .catch((err) => Err(new ProfileDataError(err.message)));
}

async function removeProfile(
  profileName: string
): Promise<Result<void, ProfileDataError>> {
  return invoke<void>('delete_profile', { profileName: profileName })
    .then(() => Ok(undefined))
    .catch((err) => Err(new ProfileDataError(err.message)));
}

async function removeProfiles(
  profileNames: string[]
): Promise<Result<void, ProfileDataError>> {
  return invoke<void>('delete_profiles', { profileNames: profileNames })
    .then(() => Ok(undefined))
    .catch((err) => Err(new ProfileDataError(err.message)));
}
