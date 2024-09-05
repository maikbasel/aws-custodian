'use client';

import React from 'react';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '@/components/ui/breadcrumb';
import { useParameters } from '@/sections/parameters/hooks/use-parameters';
import ParametersDataTable from '@/sections/parameters/components/parameters-data-table';

export default function Parameters() {
  const { parameters, isLoading, error } = useParameters();

  if (isLoading) {
    return <div>Loading...</div>; // FIXME: Make more visually appealing
  }

  if (error) {
    throw error;
  }

  return (
    <div className='flex h-full flex-col space-y-4 p-4 pt-6'>
      <Breadcrumb>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink href='/parameters'>Parameters</BreadcrumbLink>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>

      <ParametersDataTable data={parameters} />
    </div>
  );
}
