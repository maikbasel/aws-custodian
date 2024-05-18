'use client';

import React, { useEffect } from 'react';
import { useForm, useWatch } from 'react-hook-form';
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
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { outputFormats, Profile, regions } from '@/modules/profiles/domain';
import { Step, StepItem, Stepper, useStepper } from '@/components/ui/stepper';
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';
import { useSWRConfig } from 'swr';
import { isNotBlank } from '@/lib/string-utils';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface ProfileFormValue {
  name: string;
  accessKeyId: string;
  secretAccessKey: string;
  region: string;
  outputFormat: string;
}

interface ProfileFormStore {
  formData: ProfileFormValue;
  setFormData: (value: ProfileFormValue) => void;
  resetFormData: () => void;
}

const userProfileFormStore = create<ProfileFormStore>((set) => ({
  formData: {
    name: '',
    accessKeyId: '',
    secretAccessKey: '',
    region: '',
    outputFormat: '',
  },
  setFormData: (value: ProfileFormValue) => set(() => ({ formData: value })),
  resetFormData: () =>
    set(() => ({
      formData: {
        name: '',
        accessKeyId: '',
        secretAccessKey: '',
        region: '',
        outputFormat: '',
      },
    })),
}));

const StepperFormActions: React.FC = () => {
  const {
    prevStep,
    resetSteps,
    isDisabledStep,
    hasCompletedAllSteps,
    isLastStep,
    isOptionalStep,
  } = useStepper();

  return (
    <div className='flex w-full justify-end gap-2'>
      {hasCompletedAllSteps ? (
        <Button size='sm' onClick={resetSteps}>
          Reset
        </Button>
      ) : (
        <>
          <Button
            disabled={isDisabledStep}
            onClick={prevStep}
            size='sm'
            variant='secondary'
          >
            Prev
          </Button>
          <Button size='sm' type='submit'>
            {isLastStep ? 'Finish' : isOptionalStep ? 'Skip' : 'Next'}
          </Button>
        </>
      )}
    </div>
  );
};

interface StepFormProps {
  profile?: Profile;
}

interface FinalStepFormProps extends StepFormProps {
  setOpen: (value: boolean) => void;
}

const NameStepForm: React.FC<StepFormProps> = ({
  profile,
}: Readonly<StepFormProps>) => {
  const isCreate = !profile;

  const formSchema = z.object({
    name: z.string().refine(
      (arg) => {
        return isCreate ? arg !== undefined && arg.trim().length > 0 : true;
      },
      () => ({ message: 'Profile name must not be empty.' })
    ),
  });

  const { formData, setFormData } = userProfileFormStore();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    values: !isCreate // Edit
      ? {
          name: profile.name,
        }
      : undefined,
    defaultValues: isCreate
      ? {
          name: formData.name,
        }
      : undefined,
  });

  const nameWatched = useWatch({
    control: form.control,
    name: 'name',
  });

  useEffect(() => {
    setFormData({ ...formData, name: nameWatched });
  }, [nameWatched]);

  const { nextStep } = useStepper();
  function onSubmit() {
    nextStep();
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <FormField
          control={form.control}
          name='name'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Profile name</FormLabel>
              <FormControl>
                <Input
                  disabled={profile !== undefined}
                  placeholder='dev'
                  {...field}
                />
              </FormControl>
              <FormDescription>
                This is the name for your configuration settings and credentials
                profile.
              </FormDescription>
              {form.formState.errors.name && (
                <FormMessage>{form.formState.errors.name.message}</FormMessage>
              )}
            </FormItem>
          )}
        />

        <StepperFormActions />
      </form>
    </Form>
  );
};

const CredentialsStepForm: React.FC<StepFormProps> = ({
  profile,
}: Readonly<StepFormProps>) => {
  const isCreate = !profile;

  const formSchema = z.object({
    accessKeyId: z.string().trim().min(1, 'Access key ID must not be empty.'),
    secretAccessKey: z
      .string()
      .trim()
      .min(1, 'Secret access key must not be empty.'),
  });

  const { formData, setFormData } = userProfileFormStore();

  const existingAccessKeyId = profile?.credentials.access_key_id ?? '';
  const existingSecretAccessKey = profile?.credentials.secret_access_key ?? '';
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    values: !isCreate // Edit
      ? {
          accessKeyId: isNotBlank(formData.accessKeyId) // Not blank if the values was already edited.
            ? formData.accessKeyId
            : existingAccessKeyId,
          secretAccessKey: isNotBlank(formData.secretAccessKey) // Not blank if the values was already edited.
            ? formData.secretAccessKey
            : existingSecretAccessKey,
        }
      : undefined,
    defaultValues: isCreate
      ? {
          accessKeyId: formData.accessKeyId,
          secretAccessKey: formData.secretAccessKey,
        }
      : undefined,
  });
  const accessKeyIdWatched = useWatch({
    control: form.control,
    name: 'accessKeyId',
  });
  const secretAccessKeyWatched = useWatch({
    control: form.control,
    name: 'secretAccessKey',
  });
  userProfileFormStore();
  useEffect(() => {
    setFormData({
      ...formData,
      accessKeyId: accessKeyIdWatched,
      secretAccessKey: secretAccessKeyWatched,
    });
  }, [accessKeyIdWatched, secretAccessKeyWatched]);

  const { nextStep } = useStepper();

  function onSubmit() {
    nextStep();
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <FormField
          control={form.control}
          name='accessKeyId'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Access Key ID</FormLabel>
              <FormControl>
                <Input {...field} />
              </FormControl>
              <FormDescription>
                The AWS access key associated with an IAM account.
              </FormDescription>
              {form.formState.errors?.accessKeyId && (
                <FormMessage>
                  {form.formState.errors.accessKeyId.message}
                </FormMessage>
              )}
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name='secretAccessKey'
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
              {form.formState.errors?.secretAccessKey && (
                <FormMessage>
                  {form.formState.errors.secretAccessKey.message}
                </FormMessage>
              )}
            </FormItem>
          )}
        />

        <StepperFormActions />
      </form>
    </Form>
  );
};

const ConfigStepForm: React.FC<FinalStepFormProps> = ({
  profile,
  setOpen,
}: Readonly<FinalStepFormProps>) => {
  const isCreate = !profile;

  const formSchema = z.object({
    region: z.string().trim().min(1, 'Region must not be empty.'),
    outputFormat: z.string().trim().min(1, 'Output format must not be empty.'),
  });

  const { formData, setFormData, resetFormData } = userProfileFormStore();

  const existingRegion = profile?.config.region ?? '';
  const existingOutputFormat = profile?.config.output_format ?? '';
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    values: !isCreate // Edit
      ? {
          region: isNotBlank(formData.region) // Not blank if the values was already edited.
            ? formData.region
            : existingRegion,
          outputFormat: isNotBlank(formData.outputFormat) // Not blank if the values was already edited.
            ? formData.outputFormat
            : existingOutputFormat,
        }
      : undefined,
    defaultValues: isCreate
      ? {
          region: formData.region,
          outputFormat: 'json',
        }
      : undefined,
  });

  const regionWatched = useWatch({
    control: form.control,
    name: 'region',
  });
  const outputFormatWatched = useWatch({
    control: form.control,
    name: 'outputFormat',
  });
  useEffect(() => {
    setFormData({
      ...formData,
      region: regionWatched,
      outputFormat: outputFormatWatched,
    });
  }, [regionWatched, outputFormatWatched]);

  const { mutate } = useSWRConfig();

  async function onCreate() {
    invoke('create_profile', {
      profile: {
        name: formData.name,
        credentials: {
          access_key_id: formData.accessKeyId,
          secret_access_key: formData.secretAccessKey,
        },
        config: {
          region: formData.region,
          output_format: formData.outputFormat,
        },
      },
    })
      .then(() => {
        mutate('get_profiles');
      })
      .catch((reason) => console.error(reason))
      .finally(() => {
        resetFormData();
        setOpen(false);
      });
  }

  async function onEdit() {
    invoke('edit_profile', {
      profile: {
        name: formData.name,
        credentials: {
          access_key_id: formData.accessKeyId,
          secret_access_key: formData.secretAccessKey,
        },
        config: {
          region: formData.region,
          output_format: formData.outputFormat,
        },
      },
    })
      .then(() => {
        mutate('get_profiles');
      })
      .catch((reason) => console.error(reason))
      .finally(() => {
        resetFormData();
        setOpen(false);
      });
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(isCreate ? onCreate : onEdit)}
        className='space-y-8'
      >
        <FormField
          control={form.control}
          name='region'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Region</FormLabel>

              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder='Select a region' />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {regions.map((region) => (
                    <SelectItem key={region} value={region}>
                      {region}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>

              <FormDescription>
                The AWS Region whose servers you want to send your requests to.
              </FormDescription>
              {form.formState.errors.region && (
                <FormMessage>
                  {form.formState.errors.region.message}
                </FormMessage>
              )}
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name='outputFormat'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Output Format</FormLabel>

              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder='Select an output format' />
                  </SelectTrigger>
                </FormControl>
                <SelectContent defaultValue={outputFormats[0]}>
                  {outputFormats.map((outputFormat) => (
                    <SelectItem key={outputFormat} value={outputFormat}>
                      {outputFormat}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>

              <FormDescription>The output format to use.</FormDescription>
              {form.formState.errors.outputFormat && (
                <FormMessage>
                  {form.formState.errors.outputFormat.message}
                </FormMessage>
              )}
            </FormItem>
          )}
        />

        <StepperFormActions />
      </form>
    </Form>
  );
};

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
  const { resetFormData } = userProfileFormStore();

  function onOpenChange(openState: boolean) {
    setOpen(openState);
    if (open) {
      resetFormData();
    }
  }

  const steps: StepItem[] = [
    {
      label: 'Name',
    },
    {
      label: 'Credentials',
    },
    {
      label: 'Configuration',
    },
  ];

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

        <Stepper variant='circle-alt' initialStep={0} steps={steps}>
          <Step key={steps[0].label} {...steps[0]}>
            <NameStepForm profile={profile} />
          </Step>

          <Step key={steps[1].label} {...steps[1]}>
            <CredentialsStepForm profile={profile} />
          </Step>

          <Step key={steps[2].label} {...steps[2]}>
            <ConfigStepForm profile={profile} setOpen={setOpen} />
          </Step>
        </Stepper>
      </DialogContent>
    </Dialog>
  );
}
