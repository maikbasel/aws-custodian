import {
  ProfileDataError,
  ProfileDataSPI,
  ProfileSet,
} from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';

export async function getProfiles(
  profileDataSPI: ProfileDataSPI
): Promise<Result<ProfileSet, ProfileDataError>> {
  return profileDataSPI.loadProfiles();
}
