import {
  DataTableToolbar,
  SearchInputFilter,
} from '@/components/ui/data-table-toolbar';
import { DataTable } from '@/components/ui/data-table';
import { DataTablePagination } from '@/components/ui/data-table-pagination';
import React from 'react';
import { Parameter, ParameterSet } from '@/modules/parameters/core/domain';
import {
  ColumnDef,
  ColumnFiltersState,
  getCoreRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  RowSelectionState,
  SortingState,
  VisibilityState,
} from '@tanstack/table-core';
import { useReactTable } from '@tanstack/react-table';
import { Checkbox } from '@/components/ui/checkbox';
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header';

const parameterColumns: ColumnDef<Parameter>[] = [
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
    accessorKey: 'value',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Value' />
    ),
  },
  {
    accessorKey: 'version',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Version' />
    ),
  },
  {
    accessorKey: 'last_modified_date',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='Last Modified' />
    ),
  },
  {
    accessorKey: 'identifier',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title='ID' />
    ),
  },
];

type ParameterDataTableProps = {
  data: ParameterSet;
};

export default function ParametersDataTable({
  data,
}: Readonly<ParameterDataTableProps>) {
  const parameters: Parameter[] = data.parameters;

  const searchInputFilter: SearchInputFilter = {
    columnName: 'name',
    placeholder: 'Filter parameters',
  };

  const [rowSelection, setRowSelection] = React.useState<RowSelectionState>({});
  const [columnVisibility, setColumnVisibility] =
    React.useState<VisibilityState>({});
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
    []
  );
  const [sorting, setSorting] = React.useState<SortingState>([]);

  const table = useReactTable({
    data: parameters,
    columns: parameterColumns,
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

  return (
    <div className='space-y-4'>
      <DataTableToolbar
        filterableColumns={[]}
        searchInputFilter={searchInputFilter}
        table={table}
      ></DataTableToolbar>

      <DataTable table={table} />

      <DataTablePagination table={table} />
    </div>
  );
}
