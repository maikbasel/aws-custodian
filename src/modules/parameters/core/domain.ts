import { z } from 'zod';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

export const availableParameterSchema = z.object({
  names: z.string().array(),
});

export type AvailableParameter = z.infer<typeof availableParameterSchema>;

const parameterSchema = z.object({
  name: z.string(),
  value: z.string(),
  version: z.number(),
  last_modified_date: z.string().datetime().nullish(),
  identifier: z.string().nullish(),
});

export const parameterSetSchema = z.object({
  parameters: z.array(parameterSchema),
});

export type Parameter = z.infer<typeof parameterSchema>;

export type ParameterSet = z.infer<typeof parameterSetSchema>;

export interface ParameterDataSPI {
  getAvailableParameters(
    profileName: string
  ): Promise<Result<AvailableParameter, BackendError>>;

  getParameters(
    profileName: string,
    parameterNames: string[]
  ): Promise<Result<ParameterSet, BackendError>>;
}
