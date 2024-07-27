import {
  Profile,
  ProfileDataSPI,
  ProfileSet,
  profileSetSchema,
} from '@/modules/profiles/core/domain';
import { Err, Ok, Result } from 'oxide.ts';
import { invoke } from '@tauri-apps/api/tauri';
import {
  BackendError,
  backendErrorResponseSchema,
} from '@/modules/error/error';

export function createProfileDataAdapter(): ProfileDataSPI {
  return {
    loadProfiles,
    saveProfile,
    updateProfile,
    removeProfile,
    removeProfiles,
  };
}

function parseError(err: unknown) {
  const errorResponse = backendErrorResponseSchema.parse(err);
  return Err(errorResponse.error);
}

async function loadProfiles(): Promise<Result<ProfileSet, BackendError>> {
  return invoke<Record<string, never>>('get_profiles')
    .then((data) => Ok(profileSetSchema.parse(data)))
    .catch((err) => parseError(err));
}

async function saveProfile(
  profile: Profile
): Promise<Result<void, BackendError>> {
  return invoke<void>('create_profile', { profile })
    .then(() => Ok(undefined))
    .catch((err) => parseError(err));
}

async function updateProfile(
  profile: Profile
): Promise<Result<void, BackendError>> {
  return invoke<void>('edit_profile', { profile })
    .then(() => Ok(undefined))
    .catch((err) => parseError(err));
}

async function removeProfile(
  profileName: string
): Promise<Result<void, BackendError>> {
  return invoke<void>('delete_profile', { profileName: profileName })
    .then(() => Ok(undefined))
    .catch((err) => parseError(err));
}

async function removeProfiles(
  profileNames: string[]
): Promise<Result<void, BackendError>> {
  return invoke<void>('delete_profiles', { profileNames: profileNames })
    .then(() => Ok(undefined))
    .catch((err) => parseError(err));
}
