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

// TODO:
//  - Load available profiles
//  - Truncate profile names longer than 4 characters
//  - Switch profile on click in dropdown
//  - Pick default profile as default

export function ProfileNav() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant='ghost' className='relative h-10 w-10 rounded-full'>
          <Avatar className='h-10 w-10'>
            <AvatarFallback>DEV</AvatarFallback>
          </Avatar>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className='w-56' align='end' forceMount>
        <DropdownMenuGroup>
          <DropdownMenuItem>
            <div className='flex items-center justify-between space-x-4'>
              <Button
                variant='ghost'
                className='relative h-10 w-10 rounded-full'
              >
                <Avatar className='h-10 w-10'>
                  <AvatarFallback>QA</AvatarFallback>
                </Avatar>
              </Button>

              <div className='flex items-center justify-start gap-2 p-2'>
                <div className='flex flex-col space-y-1 leading-none'>
                  <p className='truncate font-medium'>QA</p>
                  <p className='w-[200px] truncate text-sm text-zinc-700'>
                    123456789
                  </p>
                </div>
              </div>
            </div>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <div className='flex items-center justify-between space-x-4'>
              <Button
                variant='ghost'
                className='relative h-10 w-10 rounded-full'
              >
                <Avatar className='h-10 w-10'>
                  <AvatarFallback>PROD</AvatarFallback>
                </Avatar>
              </Button>

              <div className='flex items-center justify-start gap-2 p-2'>
                <div className='flex flex-col space-y-1 leading-none'>
                  <p className='truncate font-medium'>PROD</p>
                  <p className='w-[200px] truncate text-sm text-zinc-700'>
                    987654321
                  </p>
                </div>
              </div>
            </div>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
