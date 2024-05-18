'use client';

import React from 'react';

import { cn } from '@/lib/css-utils';
import { Separator } from '@/components/ui/separator';
import { Button } from '@/components/ui/button';
import { ChevronRight, Settings } from 'lucide-react';
import { useSidebar } from '@/sections/dashboard/hooks/use-sidebar';
import {
  NavItem,
  SideNav,
} from '@/sections/dashboard/components/sidebar/side-nav';

export const NavItems: NavItem[] = [
  {
    title: 'Profiles',
    icon: Settings,
    href: '/',
  },
];

interface SidebarProps {
  className?: string;
}

export default function Sidebar({ className }: Readonly<SidebarProps>) {
  const { isOpen, toggle } = useSidebar();
  const [switched, setSwitched] = React.useState(false);

  const handleToggle = () => {
    setSwitched(true);
    toggle();
    setTimeout(() => setSwitched(false), 500);
  };
  return (
    <nav
      className={cn(
        'relative hidden h-screen border-r pt-16 md:block',
        switched && 'duration-500',
        isOpen ? 'w-72' : 'w-[78px]',
        className
      )}
      data-testid='sidebar-nav'
    >
      <div className='space-y-4 py-4'>
        <div className='px-3 py-2'>
          <div className='mt-3 space-y-1'>
            <SideNav
              className='text-background opacity-0 transition-all duration-300 group-hover:z-50 group-hover:ml-4 group-hover:rounded group-hover:bg-foreground group-hover:p-2 group-hover:opacity-100'
              items={NavItems}
            />
          </div>
        </div>
      </div>
      <div className='mt-30 absolute bottom-5 w-full space-y-2 px-3'>
        <Separator />
        <Button
          onClick={handleToggle}
          className={cn('h-10 w-full bg-foreground', isOpen && 'rotate-180')}
        >
          <ChevronRight className='h-4 w-4' />
        </Button>
      </div>
    </nav>
  );
}
