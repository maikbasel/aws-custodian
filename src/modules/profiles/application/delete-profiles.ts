import {
  ProfileDataError,
  ProfileDataSPI,
} from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';

export async function deleteProfiles(
  profileDataSPI: ProfileDataSPI,
  profileNames: string[]
): Promise<Result<void, ProfileDataError>> {
  return profileDataSPI.removeProfiles(profileNames);
}
