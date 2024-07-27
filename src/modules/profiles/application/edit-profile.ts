import { Profile, ProfileDataSPI } from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

export async function editProfile(
  profileDataSPI: ProfileDataSPI,
  profile: Profile
): Promise<Result<void, BackendError>> {
  return profileDataSPI.updateProfile(profile);
}
