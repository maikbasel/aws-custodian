'use client';

import React from 'react';
import Link from 'next/link';
import { cn } from '@/lib/utils';
import { ThemeToggle } from '@/sections/dashboard/components/header/theme-toggle';
import { MobileSidebar } from '@/sections/dashboard/components/header/mobile-sidebar';
import { ProfileNav } from '@/sections/dashboard/components/header/profile-nav';

export default function Header() {
  return (
    <div className='supports-backdrop-blur:bg-background/60 fixed left-0 right-0 top-0 z-20 border-b bg-background/95 backdrop-blur'>
      <nav className='flex h-16 items-center justify-between px-4'>
        <Link
          href={'/'}
          className='hidden items-center justify-between gap-2 md:flex'
        >
          <h1 className='text-lg font-semibold' data-testid='app-header-label'>
            AWS Custodian
          </h1>
        </Link>
        <div className={cn('block md:!hidden')}>
          <MobileSidebar />
        </div>

        <div className='flex items-center gap-2'>
          <ThemeToggle />
          <ProfileNav />
        </div>
      </nav>
    </div>
  );
}
