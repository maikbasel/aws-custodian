'use client';

import React, { useEffect } from 'react';
import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { BackendError } from '@/modules/error/error';

type ErrorPageProps = {
  error: BackendError & { digest?: string };
  reset: () => void;
};

// TODO: Customize further
export default function ErrorPage({ error, reset }: Readonly<ErrorPageProps>) {
  useEffect(() => {
    console.error(error);
  }, [error]);

  return (
    <div className='min-h-full px-4 py-16 sm:px-6 sm:py-24 md:grid md:place-items-center lg:px-8'>
      <div className='mx-auto max-w-max'>
        <main className='sm:flex'>
          <p className='bg-gradient-to-br from-red-400 to-yellow-600 bg-clip-text text-4xl font-bold tracking-tight text-transparent sm:text-5xl'>
            {error.code ?? 500}
          </p>
          <div className='sm:ml-6'>
            <div className='sm:border-l sm:border-gray-200 sm:pl-6'>
              <h1 className='text-4xl font-bold tracking-tight text-white sm:text-5xl'>
                Internal Server error
              </h1>
              <p className='mt-1 text-base text-gray-500'>{error.message}</p>
            </div>
            <div className='mt-10 flex space-x-3 sm:border-l sm:border-transparent sm:pl-6'>
              <Link
                href='/public'
                className='inline-flex items-center rounded-md  border-transparent  bg-gradient-to-br from-red-400 to-yellow-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-gradient-to-br hover:from-red-600 hover:to-yellow-800 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-offset-2'
              >
                Go back home
              </Link>
              <Button
                onClick={() => reset()}
                className='inline-flex items-center rounded-md border border-transparent bg-pink-100 px-4 py-2 text-sm font-medium text-black hover:bg-pink-200 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:ring-offset-2'
              >
                Try again
              </Button>
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
