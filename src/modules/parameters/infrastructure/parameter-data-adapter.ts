import {
  AvailableParameter,
  availableParameterSchema,
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
    getAvailableParameters,
    getParameters,
  };
}

async function getParameters(
  profileName: string,
  parameterNames: string[]
): Promise<Result<ParameterSet, BackendError>> {
  return invoke<Record<string, never>>('get_parameters', {
    profileName: profileName,
    parameterNames: parameterNames,
  })
    .then((data) => Ok(parameterSetSchema.parse(data)))
    .catch((reason) => {
      console.error(reason);
      const errorResponse = backendErrorResponseSchema.parse(reason);
      return Err(errorResponse.error);
    });
}

async function getAvailableParameters(
  profileName: string
): Promise<Result<AvailableParameter, BackendError>> {
  return invoke<Record<string, never>>('get_available_parameters', {
    profileName: profileName,
  })
    .then((data) => Ok(availableParameterSchema.parse(data)))
    .catch((reason) => {
      console.error(reason);
      const errorResponse = backendErrorResponseSchema.parse(reason);
      return Err(errorResponse.error);
    });
}
