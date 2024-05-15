'use client';

import { Shield, ShieldAlert, ShieldCheck } from 'lucide-react';
import { Button } from '@/components/ui/button';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';

type TestCredentialsButtonProps = {
  profile: string;
};

export default function TestCredentialsButton({
  profile,
}: Readonly<TestCredentialsButtonProps>) {
  const [validated, setValidated] = useState<boolean>(false);
  const [busy, setBusy] = useState<boolean>(false);
  const [valid, setValid] = useState<boolean>(false);

  useEffect(() => {
    setValidated(true);
    setBusy(true);

    invoke<boolean>('validate_credentials', {
      profileName: profile,
    }).then((value) => {
      setValid(value);
    });

    setBusy(false);
  }, [profile]);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          variant='ghost'
          size='icon'
          className='flex items-center'
          disabled={busy}
          onClick={() => {
            setValidated(true);
            setBusy(true);
            setTimeout(async () => {
              invoke<boolean>('validate_credentials', {
                profileName: profile,
              }).then((value) => {
                setValid(value);
              });
              setBusy(false);
            }, 1000);
          }}
        >
          {validated ? (
            valid ? (
              <>
                <ShieldCheck
                  className={`h-4 w-4 text-green-500 ${
                    busy ? 'animate-pulse' : ''
                  }`}
                />
                <span className='sr-only'>Valid</span>
              </>
            ) : (
              <>
                <ShieldAlert
                  className={`h-4 w-4 text-red-500 ${
                    busy ? 'animate-pulse' : ''
                  }`}
                />
                <span className='sr-only'>Invalid</span>
              </>
            )
          ) : (
            <>
              <Shield className='h-4 w-4 text-gray-500' />
              <span className='sr-only'>Validating...</span>
            </>
          )}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{validated ? (valid ? 'Valid' : 'Invalid') : 'Validating...'}</p>
      </TooltipContent>
    </Tooltip>
  );
}
