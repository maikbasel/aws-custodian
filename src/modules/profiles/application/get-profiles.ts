import { ProfileDataSPI, ProfileSet } from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/common/error';

export async function getProfiles(
  profileDataSPI: ProfileDataSPI
): Promise<Result<ProfileSet, BackendError>> {
  return profileDataSPI.loadProfiles();
}
