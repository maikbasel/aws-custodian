'use client';

import React from 'react';
import { ColumnDef } from '@tanstack/table-core';
import { ProfileSet } from '@/modules/profiles/domain';
import { DataTable } from '@/components/ui/data-table';
import { FileType, Globe2Icon, LucideIcon } from 'lucide-react';
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header';
import { Checkbox } from '@/components/ui/checkbox';
import {
  FilterableColumn,
  SearchInputFilter,
} from '@/components/ui/data-table-toolbar';
import TestCredentialsButton from '@/sections/profiles/components/test-credentials-button';

export type Profile = {
  name: string;
  access_key_id?: string;
  secret_access_key?: string;
  region?: string;
  output_format?: string;
};

const profileColumns: ColumnDef<Profile>[] = [
  {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && 'indeterminate')
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label='Select all'
        className='translate-y-[2px]'
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label={`Select row ${row.index + 1}`}
        className='translate-y-[2px]'
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'name',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Name' />
    ),
  },
  {
    accessorKey: 'access_key_id',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Access Key ID' />
    ),
  },
  {
    accessorKey: 'secret_access_key',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Secret Access Key' />
    ),
  },
  {
    accessorKey: 'region',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Region' />
    ),
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id));
    },
  },
  {
    accessorKey: 'output_format',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Output Format' />
    ),
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id));
    },
  },
  {
    id: 'test',
    cell: ({ row }) => {
      const profile = row.original;

      return <TestCredentialsButton profile={profile.name} />;
    },
  },
];

function flattenProfileSet(profileSet: ProfileSet): Profile[] {
  const flattenedArr = [];

  for (const [key, value] of Object.entries(profileSet.profiles)) {
    const flattenedObj = {
      name: key,
      ...value.credentials,
      ...value.config,
    };
    flattenedArr.push(flattenedObj);
  }

  return flattenedArr;
}

type ProfileDataTableProps = {
  data: ProfileSet;
};

export function ProfileDataTable({ data }: Readonly<ProfileDataTableProps>) {
  const profiles: Profile[] = flattenProfileSet(data);

  const getFilterOptions = (property: keyof Profile, icon: LucideIcon) =>
    profiles
      .filter((profile) => profile[property] !== undefined)
      .filter(
        (profile, index, array) =>
          array.findIndex((entry) => entry[property] === profile[property]) ===
          index
      )
      .map((profile) => {
        return {
          label: profile[property] as string,
          value: profile[property] as string,
          icon: icon,
        };
      });

  const regionFilterOptions = getFilterOptions('region', Globe2Icon);
  const outputFormatFilterOptions = getFilterOptions('output_format', FileType);

  const filterableColumns: FilterableColumn[] = [
    {
      title: 'Region',
      columnName: 'region',
      options: regionFilterOptions,
    },
    {
      title: 'Output Format',
      columnName: 'output_format',
      options: outputFormatFilterOptions,
    },
  ];

  const searchInputFilter: SearchInputFilter = {
    columnName: 'name',
    placeholder: 'Filter profiles',
  };

  return (
    <DataTable
      columns={profileColumns}
      data={profiles}
      searchInputFilter={searchInputFilter}
      filterableColumns={filterableColumns}
    />
  );
}
