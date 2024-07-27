import { z } from 'zod';
import { Result } from 'oxide.ts';
import { BackendError } from '@/modules/error/error';

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
  getParameters(
    profileName: string,
    pageSize: number
  ): Promise<Result<ParameterSet, BackendError>>;
}
