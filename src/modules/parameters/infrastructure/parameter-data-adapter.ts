import {
  ParameterDataSPI,
  ParameterSet,
  parameterSetSchema,
} from '@/modules/parameters/core/domain';
import { Err, Ok, Result } from 'oxide.ts';
import {
  BackendError,
  backendErrorResponseSchema,
} from '@/modules/error/error';
import { invoke } from '@tauri-apps/api/tauri';

export function createParameterDataAdapter(): ParameterDataSPI {
  return {
    getParameters,
  };
}

async function getParameters(
  profileName: string,
  pageSize: number
): Promise<Result<ParameterSet, BackendError>> {
  return invoke<Record<string, never>>('get_parameters', {
    profileName: profileName,
    pageSize: pageSize,
  })
    .then((data) => Ok(parameterSetSchema.parse(data)))
    .catch((reason) => {
      const errorResponse = backendErrorResponseSchema.parse(reason);
      return Err(errorResponse.error);
    });
}
