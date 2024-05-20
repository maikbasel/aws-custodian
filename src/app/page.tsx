'use client';

import React from 'react';
import { ProfileDataTable } from '@/sections/profiles/components/profile-data-table';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '@/components/ui/breadcrumb';
import { useProfileSet } from '@/hooks/use-profile-set';

const ProfilesPage = () => {
  const { profileSet, error, isLoading } = useProfileSet();

  if (isLoading) {
    return <div>Loading...</div>; // FIXME: Make more visually appealing
  }

  if (error) {
    throw new Error(error.message); // FIXME: Handle error
  }

  return <ProfileDataTable data={profileSet!} />;
};

export default function App() {
  return (
    <div className='flex h-full flex-col space-y-4 p-4 pt-6'>
      <Breadcrumb>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink href='/'>Profiles</BreadcrumbLink>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>

      <ProfilesPage />
    </div>
  );
}
