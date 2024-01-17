'use client';

import { DropdownMenu } from '@radix-ui/react-dropdown-menu';
import {
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import React, { useEffect, useState } from 'react';
import { ChevronDown, ChevronUp } from 'lucide-react';
import { useProfileContext } from '@/sections/dashboard/context/profile-context';
import { z } from 'zod';

interface ProfileNavItemProps {
  profileName: string;
  region: string;
  output_format: string;
  onClick: () => void;
}

export const ProfileNavItem: React.FC<ProfileNavItemProps> = ({
  profileName,
  region,
  output_format,
  onClick,
}) => (
  <DropdownMenuItem>
    <Button
      variant='ghost'
      className='relative w-full items-center justify-start gap-2 p-2'
      onClick={onClick}
    >
      <Avatar className='h-9 w-9'>
        <AvatarFallback>{profileName}</AvatarFallback>
      </Avatar>
      <div className='flex items-center justify-start gap-2 p-2'>
        <div className='flex flex-col space-y-1 leading-none'>
          <p
            className='truncate font-medium'
            data-testid={`${profileName}-profile-nav-item-region-label`}
          >
            {region}
          </p>
          <p
            className='truncate text-sm text-zinc-700'
            data-testid={`${profileName}-profile-nav-item-format-label`}
          >
            {output_format}
          </p>
        </div>
      </div>
    </Button>
  </DropdownMenuItem>
);

type Config = {
  region?: string;
  output_format?: string;
};

type Credentials = {
  access_key_id?: string;
  secret_access_key?: string;
};

type Settings = {
  credentials: Credentials;
  config: Config;
};

export type ProfileSet = {
  profiles: Record<string, Settings>;
  errors: Record<string, string[]>;
};

const profileSetSchema = z.object({
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

export function ProfileNav() {
  const [open, setOpen] = React.useState(false);
  const { data, error, isLoading } = useProfileContext();
  const [currentProfile, setCurrentProfile] = useState<string>();
  const [profileSet, setProfileSet] = useState<ProfileSet>();

  useEffect(() => {
    if (!isLoading) {
      const parsed: ProfileSet = profileSetSchema.parse(data);
      setProfileSet(parsed);

      const initialProfile = Object.keys(parsed.profiles)[0];
      console.info('1', JSON.stringify(parsed));
      setCurrentProfile(initialProfile);
    }
  }, [data, error, isLoading]);

  return (
    <DropdownMenu onOpenChange={setOpen}>
      <DropdownMenuTrigger asChild>
        {isLoading ? (
          <div>Loading...</div>
        ) : (
          <Button
            variant='outline'
            aria-expanded={open}
            aria-haspopup='true'
            className='flex items-center justify-start gap-2 p-2'
            disabled={profileSet && Object.keys(profileSet.profiles).length < 2}
            data-testid='profile-nav-trigger'
          >
            <Avatar className='h-9 w-9'>
              <AvatarFallback>{currentProfile}</AvatarFallback>
            </Avatar>

            <div className='flex items-center justify-start gap-2 p-2'>
              <div className='flex flex-col space-y-1 leading-none'>
                <p
                  className='truncate font-medium'
                  data-testid='profile-nav-trigger-region-label'
                >
                  {profileSet?.profiles?.[currentProfile!].config.region ?? '?'}
                </p>
                <p
                  className='truncate text-sm text-zinc-700'
                  data-testid='profile-nav-trigger-format-label'
                >
                  {profileSet?.profiles?.[currentProfile!].config
                    .output_format ?? '?'}
                </p>
              </div>
            </div>

            {open ? (
              profileSet && Object.keys(profileSet.profiles).length > 1 ? ( //NOSONAR
                <ChevronUp className='ml-2 h-4 w-4 shrink-0 opacity-50' />
              ) : (
                <></>
              )
            ) : profileSet && Object.keys(profileSet.profiles).length > 1 ? ( //NOSONAR
              <ChevronDown className='ml-2 h-4 w-4 shrink-0 opacity-50' />
            ) : (
              <></>
            )}
          </Button>
        )}
      </DropdownMenuTrigger>
      <DropdownMenuContent className='w-56' align='end' forceMount>
        <DropdownMenuGroup>
          {profileSet &&
            Object.entries(profileSet?.profiles)
              .filter(([profile]) => profile !== currentProfile)
              .map(([profile, settings]) => (
                <ProfileNavItem
                  key={profile}
                  profileName={profile}
                  region={settings.config.region ?? '?'}
                  output_format={settings.config.output_format ?? '?'}
                  onClick={() => setCurrentProfile(profile)}
                />
              ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
