'use client';

import Link from 'next/link';
import useNavigation from '@/hooks/Navigation';
import { Icon } from '@iconify/react';

import React from 'react';

type MenuItemProps = {
  href: string;
  isActive: boolean;
  iconActive: string;
  iconInactive: string;
  label: string;
};

const MenuItem = ({
  href,
  isActive,
  iconActive,
  iconInactive,
  label,
}: MenuItemProps) => {
  return (
    <Link
      href={href}
      className='flex w-full flex-row items-center space-x-4 px-4 py-3 duration-200 hover:bg-white/10'
    >
      {isActive ? (
        <Icon icon={iconActive} width='38' height='38' />
      ) : (
        <Icon icon={iconInactive} width='38' height='38' />
      )}
      <span
        className={`hidden pt-2 text-2xl md:flex ${
          isActive ? 'font-bold' : ''
        }`}
      >
        {label}
      </span>
    </Link>
  );
};

export default function SidebarMenu() {
  const { isHomeActive, isProfilesActive } = useNavigation();

  return (
    <nav className='fixed hidden h-full w-[120px] flex-col items-center space-y-4 border-r border-zinc-700  bg-gray-700 py-8 sm:flex md:w-[250px] md:items-start'>
      <MenuItem
        href='/'
        isActive={isHomeActive}
        iconActive='ant-design:home-filled'
        iconInactive='ant-design:home-outlined'
        label='Home'
      />
      <MenuItem
        href='/profiles'
        isActive={isProfilesActive}
        iconActive='ant-design:profile-filled'
        iconInactive='ant-design:profile-outlined'
        label='Profiles'
      />
    </nav>
  );
}
