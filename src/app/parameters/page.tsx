'use client';

import React from 'react';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
} from '@/components/ui/breadcrumb';
import ParametersDataTable from '@/sections/parameters/components/parameters-data-table';
import { useParameterSet } from '@/sections/parameters/hooks/use-parameter-set';

export default function Parameters() {
  const { parameterSet, error, isLoading } = useParameterSet();

  if (isLoading) {
    return <div>Loading...</div>; // FIXME: Make more visually appealing
  }

  if (error) {
    throw new Error(error.message);
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

      <ParametersDataTable data={parameterSet!} />
    </div>
  );
}
