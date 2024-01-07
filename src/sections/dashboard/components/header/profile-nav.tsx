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
import React from 'react';
import { ChevronDown, ChevronUp } from 'lucide-react';

// TODO:
//  - Load available profiles
//  - Truncate profile names longer than 4 characters
//  - Switch profile on click in dropdown
//  - Pick default profile as default

interface ProfileNavItemProps {
  profileName: string;
  accessKeyId: string;
  region: string;
}

export const ProfileNavItem: React.FC<ProfileNavItemProps> = ({
  profileName,
  accessKeyId,
  region,
}) => (
  <DropdownMenuItem>
    <Button
      variant='ghost'
      className='relative w-full items-center justify-start gap-2 p-2'
    >
      <Avatar className='h-9 w-9'>
        <AvatarFallback>{profileName}</AvatarFallback>
      </Avatar>
      <div className='flex items-center justify-start gap-2 p-2'>
        <div className='flex flex-col space-y-1 leading-none'>
          <p className='truncate font-medium'>{accessKeyId}</p>
          <p className='truncate text-sm text-zinc-700'>{region}</p>
        </div>
      </div>
    </Button>
  </DropdownMenuItem>
);

export function ProfileNav() {
  const [open, setOpen] = React.useState(false);

  return (
    <DropdownMenu onOpenChange={setOpen}>
      <DropdownMenuTrigger asChild>
        <Button
          variant='outline'
          aria-expanded={open}
          aria-haspopup='true'
          className='flex items-center justify-start gap-2 p-2'
        >
          <Avatar className='h-9 w-9'>
            <AvatarFallback>DEV</AvatarFallback>
          </Avatar>

          <div className='flex items-center justify-start gap-2 p-2'>
            <div className='flex flex-col space-y-1 leading-none'>
              <p className='truncate font-medium'>123456789</p>
              <p className='truncate text-sm text-zinc-700'>eu-west-1</p>
            </div>
          </div>

          {open ? (
            <ChevronUp className='ml-2 h-4 w-4 shrink-0 opacity-50' />
          ) : (
            <ChevronDown className='ml-2 h-4 w-4 shrink-0 opacity-50' />
          )}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className='w-56' align='end' forceMount>
        <DropdownMenuGroup>
          <ProfileNavItem
            profileName='QA'
            accessKeyId='124816321'
            region='eu-west-1'
          />
          <ProfileNavItem
            profileName='PROD'
            accessKeyId='987654321'
            region='eu-west-1'
          />
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
