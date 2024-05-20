import {
  ProfileDataError,
  ProfileDataSPI,
} from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';

export async function deleteProfile(
  profileDataSPI: ProfileDataSPI,
  profileName: string
): Promise<Result<void, ProfileDataError>> {
  return profileDataSPI.removeProfile(profileName);
}
