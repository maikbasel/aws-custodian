'use client';

import React from 'react';
import {
  ColumnDef,
  ColumnFiltersState,
  getCoreRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  Row,
  RowSelectionState,
  SortingState,
  VisibilityState,
} from '@tanstack/table-core';
import { Profile, ProfileSet } from '@/modules/profiles/core/domain';
import { DataTable } from '@/components/ui/data-table';
import { FileType, Globe2Icon, MoreHorizontal } from 'lucide-react';
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header';
import { Checkbox } from '@/components/ui/checkbox';
import {
  DataTableToolbar,
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
import { useSWRConfig } from 'swr';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import ProfileFormDialog from '@/sections/profiles/components/profile-form-dialog';
import { useReactTable } from '@tanstack/react-table';
import { DataTablePagination } from '@/components/ui/data-table-pagination';
import ProfileActionsButton from '@/sections/profiles/components/profile-actions-button';
import { useProfileForm } from '@/sections/profiles/hooks/use-profile-form';
import { toast } from '@/components/ui/use-toast';

const RowAction: React.FC<{ row: Row<Profile> }> = ({ row }) => {
  const profile = row.original;
  const { mutate } = useSWRConfig();
  const [showDeleteDialog, setShowDeleteDialog] = React.useState(false);
  const [showUpdateDialog, setShowUpdateDialog] = React.useState(false);
  const { deleteProfile } = useProfileForm();

  async function onDelete() {
    deleteProfile(profile.name)
      .then((result) => {
        if (result.isOk()) {
          mutate('get_profiles');
        } else {
          const backendError = result.unwrapErr();
          toast({
            variant: 'destructive',
            title: `Deleting profile ${profile.name} failed!`,
            description: `${backendError.code}: ${backendError.message}`,
          });
        }
      })
      .finally(() => setShowDeleteDialog(false));
  }

  return (
    <>
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
          <DropdownMenuItem onSelect={() => setShowUpdateDialog(true)}>
            Edit {profile.name} profile
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => setShowDeleteDialog(true)}>
            Delete {profile.name} profile
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <ProfileFormDialog
        profile={profile}
        open={showUpdateDialog}
        setOpen={setShowUpdateDialog}
      />
      <AlertDialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the{' '}
              <strong>&nbsp;{profile.name}</strong> profile as well as it&apos;s{' '}
              corresponding configuration settings and credentials.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={onDelete}>Continue</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
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
    accessorKey: 'Status',
    cell: ({ row }) => {
      const profile = row.original;

      return <TestCredentialsButton profile={profile.name} />;
    },
  },
  {
    id: 'actions',
    cell: RowAction,
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

  const [rowSelection, setRowSelection] = React.useState<RowSelectionState>({});
  const [columnVisibility, setColumnVisibility] =
    React.useState<VisibilityState>({});
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
    []
  );
  const [sorting, setSorting] = React.useState<SortingState>([]);

  const table = useReactTable({
    data: profiles,
    columns: profileColumns,
    state: {
      sorting,
      columnVisibility,
      rowSelection,
      columnFilters,
    },
    enableRowSelection: true,
    onRowSelectionChange: setRowSelection,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    onColumnVisibilityChange: setColumnVisibility,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFacetedRowModel: getFacetedRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
  });

  const selectedRows = table
    .getSelectedRowModel()
    .rows.map(({ original }) => original);

  const [showCreateDialog, setShowCreateDialog] = React.useState(false);

  return (
    <div className='space-y-4'>
      <DataTableToolbar
        filterableColumns={filterableColumns}
        searchInputFilter={searchInputFilter}
        table={table}
      >
        <Button
          variant='outline'
          size='sm'
          className='ml-auto hidden h-8 border-dashed lg:flex'
          onClick={() => setShowCreateDialog(true)}
        >
          Create Profile
        </Button>
        <ProfileFormDialog
          open={showCreateDialog}
          setOpen={setShowCreateDialog}
        />

        <ProfileActionsButton selectedRows={selectedRows} />
      </DataTableToolbar>

      <DataTable table={table} />

      <DataTablePagination table={table} />
    </div>
  );
}
