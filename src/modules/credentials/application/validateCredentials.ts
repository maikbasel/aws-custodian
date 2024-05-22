import { CredentialsDataSPI } from '@/modules/credentials/core/domain';

export async function validateCredentials(
  credentialsDataSPI: CredentialsDataSPI,
  profileName: string
) {
  return credentialsDataSPI.validateCredentials(profileName);
}
