'use client';

import React from 'react';
import { useProfileContext } from '@/sections/dashboard/context/profile-context';
import { ProfileDataTable } from '@/sections/profiles/components/profile-data-table';
import { ProfileSet, profileSetSchema } from '@/modules/profiles/domain';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '@/components/ui/breadcrumb';

export default function Profiles() {
  const { data, error, isLoading } = useProfileContext();

  if (isLoading) {
    return <div>Loading...</div>; // FIXME: Make more visually appealing
  }

  if (error) {
    throw new Error(error.message); // FIXME: Handle error
  }

  const parsed: ProfileSet = profileSetSchema.parse(data);

  return (
    <div className='flex h-full flex-col space-y-4 p-4 pt-6'>
      <Breadcrumb>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink href='/'>Profiles</BreadcrumbLink>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>

      <ProfileDataTable data={parsed} />
    </div>
  );
}
