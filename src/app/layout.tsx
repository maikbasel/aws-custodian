import './globals.css';
import type { Metadata } from 'next';
import React, { ReactNode } from 'react';
import Sidebar from '@/sections/dashboard/components/sidebar/sidebar';
import Header from '@/sections/dashboard/components/header/header';
import { ThemeProvider } from '@/sections/dashboard/components/header/theme-provider';
import { TooltipProvider } from '@/components/ui/tooltip';
import { DIContextProvider } from '@/context/di-context';

export const metadata: Metadata = {
  title: 'Create Next App',
  description: 'Generated by create next app',
};

type RootLayoutProps = {
  children: ReactNode;
};

export default function RootLayout({ children }: Readonly<RootLayoutProps>) {
  return (
    <html lang='en' suppressHydrationWarning>
      <body>
        <ThemeProvider
          attribute='class'
          defaultTheme='system'
          enableSystem
          disableTransitionOnChange
        >
          <TooltipProvider>
            <DIContextProvider>
              <Header />
              <div className='flex h-screen border-collapse overflow-hidden'>
                <Sidebar />
                <main className='flex-1 overflow-y-auto overflow-x-hidden bg-secondary/10 pb-1 pt-16'>
                  {children}
                </main>
              </div>
            </DIContextProvider>
          </TooltipProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
