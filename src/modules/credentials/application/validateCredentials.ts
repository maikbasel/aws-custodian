import { CredentialsDataSPI } from '@/modules/credentials/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/common/error';

export async function validateCredentials(
  credentialsDataSPI: CredentialsDataSPI,
  profileName: string
): Promise<Result<boolean, BackendError>> {
  return credentialsDataSPI.validateCredentials(profileName);
}
