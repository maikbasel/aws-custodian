import {
  AvailableParameter,
  ParameterDataSPI,
} from '@/modules/parameters/core/domain';
import { BackendError } from '@/modules/error/error';
import { Result } from 'oxide.ts';

export async function getAvailableParameters(
  parameterDataSPI: ParameterDataSPI,
  profileName: string
): Promise<Result<AvailableParameter, BackendError>> {
  return parameterDataSPI.getAvailableParameters(profileName);
}
