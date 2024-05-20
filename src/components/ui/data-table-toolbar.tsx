'use client';

import React from 'react';
import { Cross2Icon } from '@radix-ui/react-icons';
import { Table } from '@tanstack/react-table';
import { DataTableFacetedFilter } from './data-table-faceted-filter';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { DataTableViewOptions } from '@/components/ui/data-table-view-options';
import { type LucideIcon } from 'lucide-react';
import ProfileFormDialog from '@/sections/profiles/components/profile-form-dialog';

interface FilterOption {
  value: string;
  label: string;
  icon: LucideIcon;
}

export interface FilterableColumn {
  columnName: string;
  title: string;
  options: FilterOption[];
}

export interface SearchInputFilter {
  placeholder: string;
  columnName: string;
}

export interface DataTableToolbarProps<TData> {
  table: Table<TData>;
  searchInputFilter: SearchInputFilter;
  filterableColumns: FilterableColumn[];
  children: React.ReactNode;
}

export function DataTableToolbar<TData>({
  table,
  searchInputFilter,
  filterableColumns,
  children,
}: Readonly<DataTableToolbarProps<TData>>) {
  const isFiltered = table.getState().columnFilters.length > 0;

  const [showCreateDialog, setShowCreateDialog] = React.useState(false);

  return (
    <div className='flex items-center justify-between'>
      <div className='flex flex-1 items-center space-x-2'>
        <Input
          placeholder={searchInputFilter.placeholder}
          value={
            (table
              .getColumn(searchInputFilter.columnName)
              ?.getFilterValue() as string) ?? ''
          }
          onChange={(event) =>
            table
              .getColumn(searchInputFilter.columnName)
              ?.setFilterValue(event.target.value)
          }
          className='h-8 w-[150px] lg:w-[250px]'
        />
        {filterableColumns.map((filterableColumn) => {
          const column = table.getColumn(filterableColumn.columnName);
          if (column) {
            return (
              <DataTableFacetedFilter
                key={filterableColumn.columnName}
                column={column}
                title={filterableColumn.title}
                options={filterableColumn.options}
              />
            );
          }
        })}
        {isFiltered && (
          <Button
            variant='ghost'
            onClick={() => table.resetColumnFilters()}
            className='h-8 px-2 lg:px-3'
          >
            Reset
            <Cross2Icon className='ml-2 h-4 w-4' />
          </Button>
        )}
      </div>
      <div className='flex flex-1 items-center space-x-2'>
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

        {children}

        <DataTableViewOptions table={table} />
      </div>
    </div>
  );
}
