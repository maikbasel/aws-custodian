'use client';

import React from 'react';
import { ColumnDef } from '@tanstack/table-core';
import { Profile, ProfileSet } from '@/modules/profiles/domain';
import { DataTable } from '@/components/ui/data-table';
import { FileType, Globe2Icon, MoreHorizontal } from 'lucide-react';
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header';
import { Checkbox } from '@/components/ui/checkbox';
import {
  FilterableColumn,
  SearchInputFilter,
} from '@/components/ui/data-table-toolbar';
import TestCredentialsButton from '@/sections/profiles/components/test-credentials-button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';

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
    id: 'access_key_id',
    accessorKey: 'credentials.access_key_id',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Access Key ID' />
    ),
  },
  {
    id: 'secret_access_key',
    accessorKey: 'credentials.secret_access_key',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Secret Access Key' />
    ),
  },
  {
    id: 'region',
    accessorKey: 'config.region',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Region' />
    ),
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id));
    },
  },
  {
    id: 'output_format',
    accessorKey: 'config.output_format',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Output Format' />
    ),
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id));
    },
  },
  {
    id: 'test',
    accessorKey: 'status',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Status' />
    ),
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id));
    },
    cell: ({ row }) => {
      const profile = row.original;
      return <TestCredentialsButton profile={profile.name} />;
    },
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      const profile = row.original;

      return (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant='ghost' className='h-8 w-8 p-0'>
              <span className='sr-only'>Open menu</span>
              <MoreHorizontal className='h-4 w-4' />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align='end'>
            <DropdownMenuLabel>Actions</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>Update {profile.name} profile</DropdownMenuItem>
            <DropdownMenuItem>Delete {profile.name} profile</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      );
    },
  },
];

type ProfileDataTableProps = {
  data: ProfileSet;
};

export function ProfileDataTable({ data }: Readonly<ProfileDataTableProps>) {
  const profiles: Profile[] = data.profiles;

  const regionFilterOptions = profiles
    .filter((profile) => profile.config.region !== undefined)
    .map((profile) => {
      return {
        label: profile.config.region!,
        value: profile.config.region!,
        icon: Globe2Icon,
      };
    });
  const outputFormatFilterOptions = profiles
    .filter((profile) => profile !== undefined)
    .filter((profile) => profile.config.output_format !== undefined)
    .map((profile) => {
      return {
        label: profile.config.output_format!,
        value: profile.config.output_format!,
        icon: FileType,
      };
    });

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
