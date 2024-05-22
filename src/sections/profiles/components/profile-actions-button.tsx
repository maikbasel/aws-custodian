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
import { Profile } from '@/modules/profiles/core/domain';
import { mutate } from 'swr';
import { useProfileForm } from '@/sections/profiles/hooks/use-profile-form';
import { toast } from '@/components/ui/use-toast';

interface DataTableActionsButtonProps {
  selectedRows: Profile[];
}

export default function ProfileActionsButton({
  selectedRows,
}: Readonly<DataTableActionsButtonProps>) {
  const [showDeleteDialog, setShowDeleteDialog] = React.useState(false);
  const [showUpdateDialog, setShowUpdateDialog] = React.useState(false);
  const [showCreateDialog, setShowCreateDialog] = React.useState(false);
  const { deleteProfiles } = useProfileForm();

  async function onDelete() {
    const profileNames = selectedRows.map((row) => row.name);

    deleteProfiles(profileNames)
      .then((result) => {
        if (result.isOk()) {
          mutate('get_profiles');
        } else {
          const backendError = result.unwrapErr();
          toast({
            variant: 'destructive',
            title: `Deleting profiles ${profileNames.join(', ')} failed!`,
            description: `${backendError.code}: ${backendError.message}`,
          });
        }
      })
      .catch((reason) => console.error(reason))
      .finally(() => setShowDeleteDialog(false));
  }

  return (
    <>
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
          <DropdownMenuItem onSelect={() => setShowCreateDialog(true)}>
            Create new
          </DropdownMenuItem>
          <DropdownMenuItem
            onSelect={() => setShowUpdateDialog(true)}
            disabled={!selectedRows || selectedRows?.length !== 1}
          >
            Edit
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            onSelect={() => setShowDeleteDialog(true)}
            disabled={!selectedRows || selectedRows?.length === 0}
          >
            Delete all
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <ProfileFormDialog
        open={showCreateDialog}
        setOpen={setShowCreateDialog}
      />

      <ProfileFormDialog
        profile={selectedRows[0]}
        open={showUpdateDialog}
        setOpen={setShowUpdateDialog}
      />

      <AlertDialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the{' '}
              <strong>&nbsp;ALL</strong> selected profiles as well as it&apos;s{' '}
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
}
