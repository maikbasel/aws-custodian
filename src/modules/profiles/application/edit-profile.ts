import {
  Profile,
  ProfileDataError,
  ProfileDataSPI,
} from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';

export async function editProfile(
  profileDataSPI: ProfileDataSPI,
  profile: Profile
): Promise<Result<void, ProfileDataError>> {
  return profileDataSPI.updateProfile(profile);
}
