'use client';

import {
  ShieldAlert,
  ShieldCheck,
  ShieldEllipsis,
  ShieldX,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import React, { useEffect, useState } from 'react';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useProfileForm } from '@/sections/profiles/hooks/use-profile-form';
import { useToast } from '@/components/ui/use-toast';
import { BackendError } from '@/modules/error/error';

type TestCredentialsButtonProps = {
  profile: string;
};

export default function TestCredentialsButton({
  profile,
}: Readonly<TestCredentialsButtonProps>) {
  const [validated, setValidated] = useState<boolean>(false);
  const [busy, setBusy] = useState<boolean>(false);
  const [valid, setValid] = useState<boolean>(false);
  const [failed, setFailed] = useState<boolean>(false);
  const { toast } = useToast();
  const { validateCredentials } = useProfileForm();

  useEffect(() => {
    setBusy(true);

    validateCredentials(profile)
      .then((value) => {
        if (value.isOk()) {
          setValid(value.unwrap());
          setValidated(true);
        } else {
          const backendError = value.unwrapErr();
          toast({
            variant: 'destructive',
            title: `Credentials validation failed! There is something wrong with profile ${profile}.`,
            description: `${backendError.code}: ${backendError.message}`,
          });
          setFailed(true);
        }
      })
      .finally(() => {
        setBusy(false);
        setValidated(true);
      });
  }, [profile]);

  const toastError = (backendError: BackendError) => {
    toast({
      variant: 'destructive',
      title: `Credentials validation failed! There is something wrong with the profile "${profile}".`,
      description: `${backendError.code}: ${backendError.message}`,
    });
  };

  const onClick = () => {
    setBusy(true);

    validateCredentials(profile)
      .then((value) => {
        const backendError = value.unwrapErr();
        if (value.isOk()) {
          setValid(value.unwrap());
        } else {
          toastError(backendError);
          setFailed(true);
        }
      })
      .finally(() => setBusy(false));
  };

  const renderIcon = (validated: boolean, valid: boolean, busy: boolean) => {
    if (!validated) {
      return (
        <>
          <ShieldEllipsis
            className={`h-4 w-4 text-gray-500 ${busy ? 'animate-pulse' : ''}`}
          />
          <span className='sr-only'>Validating...</span>
        </>
      );
    }

    if (valid) {
      return (
        <>
          <ShieldCheck
            className={`h-4 w-4 text-green-500 ${busy ? 'animate-pulse' : ''}`}
          />
          <span className='sr-only'>Valid</span>
        </>
      );
    }

    if (failed) {
      return (
        <>
          <ShieldX
            className={`h-4 w-4 text-red-500 ${busy ? 'animate-pulse' : ''}`}
          />
          <span className='sr-only'>Failed</span>
        </>
      );
    }

    return (
      <>
        <ShieldAlert
          className={`h-4 w-4 text-yellow-500 ${busy ? 'animate-pulse' : ''}`}
        />
        <span className='sr-only'>Invalid</span>
      </>
    );
  };

  const renderTooltipContent = (validated: boolean, valid: boolean) => {
    if (!validated) {
      return 'Validating...';
    }

    if (failed) {
      return 'Failed';
    }

    return valid ? 'Valid' : 'Invalid';
  };

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          variant='ghost'
          size='icon'
          className='flex items-center'
          disabled={busy}
          onClick={onClick}
        >
          {renderIcon(validated, valid, busy)}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{renderTooltipContent(validated, valid)}</p>
      </TooltipContent>
    </Tooltip>
  );
}
