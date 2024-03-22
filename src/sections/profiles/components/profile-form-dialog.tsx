'use client';

import React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
// see https://stackoverflow.com/a/77264549
import { invoke } from '@tauri-apps/api/tauri';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useSWRConfig } from 'swr';
import { Profile } from '@/modules/profiles/domain';

interface ProfileFormProps {
  profile?: Profile;
  open: boolean;
  setOpen: (open: boolean) => void;
}

export default function ProfileFormDialog({
  profile,
  open,
  setOpen,
}: Readonly<ProfileFormProps>) {
  const isCreate = !profile;

  const formSchema = z.object({
    name: z.string().min(1, 'Profile name must not be empty.'),
    credentials: z.object({
      accessKeyId: z.string().min(1, 'Access key ID must not be empty.'),
      secretAccessKey: z
        .string()
        .min(1, 'Secret access key must not be empty.'),
    }),
    config: z.object({
      region: z.string().min(1, 'Region must not be empty.'),
      outputFormat: z.string().min(1, 'Output format must not be empty.'),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    values: !isCreate
      ? {
          name: profile?.name ?? '',
          config: {
            region: profile?.config.region ?? '',
            outputFormat: profile?.config.output_format ?? '',
          },
          credentials: {
            accessKeyId: profile?.credentials.access_key_id ?? '',
            secretAccessKey: profile?.credentials.secret_access_key ?? '',
          },
        }
      : undefined,
    defaultValues: isCreate
      ? {
          config: {
            outputFormat: 'json',
          },
        }
      : undefined,
  });

  const { mutate } = useSWRConfig();

  async function onCreate(values: z.infer<typeof formSchema>) {
    invoke('create_profile', {
      profile: {
        name: values.name,
        credentials: {
          access_key_id: values.credentials.accessKeyId,
          secret_access_key: values.credentials.secretAccessKey,
        },
        config: {
          region: values.config.region,
          output_format: values.config.outputFormat,
        },
      },
    }).then(() => {
      mutate('get_profiles');
      form.reset();
      setOpen(false);
    });
  }

  async function onEdit(values: z.infer<typeof formSchema>) {
    console.info(values);
    invoke('edit_profile', {
      profile: {
        name: values.name,
        credentials: {
          access_key_id: values.credentials.accessKeyId,
          secret_access_key: values.credentials.secretAccessKey,
        },
        config: {
          region: values.config.region,
          output_format: values.config.outputFormat,
        },
      },
    })
      .then(() => {
        mutate('get_profiles');
        form.reset();
        setOpen(false);
      })
      .catch((reason) => console.error(reason));
  }

  function onOpenChange(openState: boolean) {
    form.reset();
    setOpen(openState);
  }

  return (
    <Dialog open={open} onOpenChange={(openState) => onOpenChange(openState)}>
      <DialogContent className='sm:max-w-[425px]'>
        <DialogHeader>
          <DialogTitle>Create profile</DialogTitle>
          <DialogDescription>
            Provide your AWS credentials, configuration and a name for the new
            profile here. Click save when you&apos;re done.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(isCreate ? onCreate : onEdit)}
            className='space-y-8'
          >
            <FormField
              control={form.control}
              name='name'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Profile name</FormLabel>
                  <FormControl>
                    <Input placeholder='dev' {...field} />
                  </FormControl>
                  <FormDescription>
                    This is the name for your configuration settings and
                    credentials profile.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='credentials.accessKeyId'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Access Key ID</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormDescription>
                    The AWS access key associated with an IAM account.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='credentials.secretAccessKey'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Secret Access Key</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormDescription>
                    The secret key associated with the access key. This is
                    essentially the &quot;password&quot; for the access key.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='config.region'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Region</FormLabel>
                  <FormControl>
                    <Input placeholder='eu-west-1' {...field} />
                  </FormControl>
                  <FormDescription>
                    The AWS Region whose servers you want to send your requests
                    to.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name='config.outputFormat'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Output Format</FormLabel>
                  <FormControl>
                    <Input placeholder='json' {...field} />
                  </FormControl>
                  <FormDescription>The output format to use.</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <DialogFooter>
              <Button type='submit'>Submit</Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
