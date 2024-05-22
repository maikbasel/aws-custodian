import { Err, Ok, Result } from 'oxide.ts';
import { CredentialsDataSPI } from '@/modules/credentials/core/domain';
import { invoke } from '@tauri-apps/api/tauri';
import {
  BackendError,
  backendErrorResponseSchema,
} from '@/modules/common/error';

export function createCredentialsDataAdapter(): CredentialsDataSPI {
  return {
    validateCredentials,
  };
}

async function validateCredentials(
  profileName: string
): Promise<Result<boolean, BackendError>> {
  return invoke<boolean>('validate_credentials', {
    profileName: profileName,
  })
    .then((data) => Ok(data))
    .catch((err) => {
      const errorResponse = backendErrorResponseSchema.parse(err);
      return Err(errorResponse.error);
    });
}
