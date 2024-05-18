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
import React, { useEffect } from 'react';
import { ChevronDown, ChevronUp } from 'lucide-react';
import { Profile } from '@/modules/profiles/domain';
import { useCurrentProfile } from '@/sections/dashboard/stores/use-current-profile';
import { useProfileSet } from '@/sections/dashboard/hooks/use-profile-set';

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

export function ProfileNav() {
  const [open, setOpen] = React.useState(false);
  const { profileSet, error, isLoading } = useProfileSet();
  const { current, setCurrent } = useCurrentProfile();

  useEffect(() => {
    if (!isLoading) {
      const defaultProfile = profileSet!.profiles.find(
        (value: Profile) => value.name === 'default'
      );
      const initialProfile: Profile = defaultProfile
        ? defaultProfile
        : profileSet!.profiles[0];
      setCurrent(initialProfile.name);
    }
  }, [profileSet, error, isLoading, setCurrent]);

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
            disabled={profileSet && profileSet.profiles.length < 2}
            data-testid='profile-nav-trigger'
          >
            <Avatar className='h-9 w-9'>
              <AvatarFallback>{current}</AvatarFallback>
            </Avatar>

            <div className='flex items-center justify-start gap-2 p-2'>
              <div className='flex flex-col space-y-1 leading-none'>
                <p
                  className='truncate font-medium'
                  data-testid='profile-nav-trigger-region-label'
                >
                  {profileSet?.profiles?.find(
                    (profile: Profile) => profile.name === current
                  )?.config.region ?? '?'}
                </p>
                <p
                  className='truncate text-sm text-zinc-700'
                  data-testid='profile-nav-trigger-format-label'
                >
                  {profileSet?.profiles?.find(
                    (profile: Profile) => profile.name === current
                  )?.config.output_format ?? '?'}
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
          {profileSet?.profiles
            .filter((profile: Profile) => profile.name !== current)
            .map((profile: Profile) => (
              <ProfileNavItem
                key={profile.name}
                profileName={profile.name}
                region={profile.config.region ?? '?'}
                output_format={profile.config.output_format ?? '?'}
                onClick={() => setCurrent(profile.name)}
              />
            ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
