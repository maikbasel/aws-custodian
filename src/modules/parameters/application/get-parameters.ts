import {
  ParameterDataSPI,
  ParameterSet,
} from '@/modules/parameters/core/domain';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

export async function getParameters(
  parameterDataSPI: ParameterDataSPI,
  profileName: string,
  parameterNames: string[]
): Promise<Result<ParameterSet, BackendError>> {
  return parameterDataSPI.getParameters(profileName, parameterNames);
}
