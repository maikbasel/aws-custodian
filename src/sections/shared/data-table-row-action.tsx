'use client';

import React from 'react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import { MoreHorizontal } from 'lucide-react';
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

type DialogAction = {
  description: string;
  onOpenDialog: (open: boolean) => void;
};

type DeleteDialogAction = DialogAction & {
  text: React.ReactNode;
  handler: () => void;
  isOpen: boolean;
};

type DataTableRowActionProps = {
  editAction: DialogAction;
  deleteAction: DeleteDialogAction;
  children?: React.ReactNode;
};

export default function DataTableRowAction({
  editAction,
  deleteAction,
  children,
}: Readonly<DataTableRowActionProps>) {
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
          <DropdownMenuItem onSelect={() => editAction.onOpenDialog(true)}>
            {editAction.description}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => deleteAction.onOpenDialog(true)}>
            {deleteAction.description}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {children}

      <AlertDialog
        open={deleteAction.isOpen}
        onOpenChange={deleteAction.onOpenDialog}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>{deleteAction.text}</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={deleteAction.handler}>
              Continue
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
