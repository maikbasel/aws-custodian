import { z } from 'zod';
import { Result } from 'oxide.ts';
import Error from 'next/error';

export const regions = [
  'af-south-1',
  'ap-east-1',
  'ap-northeast-1',
  'ap-northeast-2',
  'ap-northeast-3',
  'ap-south-1',
  'ap-southeast-1',
  'ap-southeast-2',
  'ap-southeast-3',
  'ca-central-1',
  'eu-central-1',
  'eu-north-1',
  'eu-south-1',
  'eu-west-1',
  'eu-west-2',
  'eu-west-3',
  'me-south-1',
  'sa-east-1',
  'us-east-1',
  'us-east-2',
  'us-west-1',
  'us-west-2',
] as const;

export const outputFormats = [
  'json',
  'yaml',
  'yaml-stream ',
  'text',
  'table',
] as const;

const profileSchema = z.object({
  name: z.string(),
  credentials: z.object({
    access_key_id: z.string().optional().nullish(),
    secret_access_key: z.string().optional().nullish(), // FIXME: Use secure string
  }),
  config: z.object({
    region: z.string().optional().nullish(),
    output_format: z.string().optional().nullish(),
  }),
});

export type Profile = z.infer<typeof profileSchema>;

export const profileSetSchema = z.object({
  profiles: z.array(profileSchema),
});

export type ProfileSet = z.infer<typeof profileSetSchema>;

export class ProfileDataError extends Error {}

export interface ProfileDataSPI {
  loadProfiles(): Promise<Result<ProfileSet, ProfileDataError>>;

  saveProfile(profile: Profile): Promise<Result<void, ProfileDataError>>;

  updateProfile(profile: Profile): Promise<Result<void, ProfileDataError>>;

  removeProfile(profileName: string): Promise<Result<void, ProfileDataError>>;

  removeProfiles(
    profileNames: string[]
  ): Promise<Result<void, ProfileDataError>>;
}
