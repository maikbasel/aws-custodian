import { ProfileDataSPI } from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

export async function deleteProfiles(
  profileDataSPI: ProfileDataSPI,
  profileNames: string[]
): Promise<Result<void, BackendError>> {
  return profileDataSPI.removeProfiles(profileNames);
}
