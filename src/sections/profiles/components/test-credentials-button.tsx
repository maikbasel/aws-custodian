'use client';

import { Shield, ShieldAlert, ShieldCheck } from 'lucide-react';
import { Button } from '@/components/ui/button';
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

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
  }, []);

  return (
    <Button
      variant='outline'
      size='icon'
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
          <ShieldCheck className={`h-4 w-4 ${busy ? 'animate-pulse' : ''}`} />
        ) : (
          <ShieldAlert className={`h-4 w-4 ${busy ? 'animate-pulse' : ''}`} />
        )
      ) : (
        <Shield className='h-4 w-4' />
      )}
    </Button>
  );
}
