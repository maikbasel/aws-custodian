'use client';

import React from 'react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import { ChevronDown } from 'lucide-react';

interface DataTableActionsButtonProps<TData> {
  selectedRows: TData[];
}

export default function DataTableActionsButton<TData>({
  selectedRows,
}: DataTableActionsButtonProps<TData>) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant='outline'
          size='sm'
          className='ml-auto hidden h-8 border-dashed lg:flex'
        >
          <ChevronDown className='mr-2 h-4 w-4' />
          Actions
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align='end' className='w-[150px]'>
        <DropdownMenuItem>
          Create new
        </DropdownMenuItem>
        <DropdownMenuItem
          disabled={!selectedRows || selectedRows?.length !== 1}
        >
          Edit
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem
          disabled={!selectedRows || selectedRows?.length === 0}
        >
          Delete all
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
