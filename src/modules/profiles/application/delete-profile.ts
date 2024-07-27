import { ProfileDataSPI } from '@/modules/profiles/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

export async function deleteProfile(
  profileDataSPI: ProfileDataSPI,
  profileName: string
): Promise<Result<void, BackendError>> {
  return profileDataSPI.removeProfile(profileName);
}
