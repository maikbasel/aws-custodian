import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/common/error';

export interface CredentialsDataSPI {
  validateCredentials(
    profileName: string
  ): Promise<Result<boolean, BackendError>>;
}
