import { Profile, ProfileDataSPI } from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/common/error';

export async function createProfile(
  profileDataSPI: ProfileDataSPI,
  profile: Profile
): Promise<Result<void, BackendError>> {
  return profileDataSPI.saveProfile(profile);
}
