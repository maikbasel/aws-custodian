import { z } from 'zod';

const backendErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});
export const backendErrorResponseSchema = z.object({
  error: backendErrorSchema,
});

export type BackendError = z.infer<typeof backendErrorSchema>;
