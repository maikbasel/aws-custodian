import { z } from 'zod';

const profileSchema = z.object({
  name: z.string(),
  credentials: z.object({
    access_key_id: z.string().optional(),
    secret_access_key: z.string().optional(), // FIXME: Use secure string
  }),
  config: z.object({
    region: z.string().optional(),
    output_format: z.string().optional(),
  }),
  valid: z.boolean().optional(),
});

export type Profile = z.infer<typeof profileSchema>;

export const profileSetSchema = z.object({
  profiles: z.array(profileSchema),
  errors: z.record(z.string().array()),
});

export type ProfileSet = z.infer<typeof profileSetSchema>;
