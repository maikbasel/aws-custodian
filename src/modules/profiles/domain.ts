import { z } from 'zod';

export const profileSetSchema = z.object({
  profiles: z.record(
    z.object({
      credentials: z.object({
        access_key_id: z.string().optional(),
        secret_access_key: z.string().optional(),
      }),
      config: z.object({
        region: z.string().optional(),
        output_format: z.string().optional(),
      }),
    })
  ),
  errors: z.record(z.string().array()),
});

export type ProfileSet = z.infer<typeof profileSetSchema>;
