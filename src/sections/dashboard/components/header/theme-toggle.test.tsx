import React from 'react';
import { render, screen } from '@testing-library/react';
import { ThemeToggle } from './theme-toggle';
import { ThemeProvider } from 'next-themes';
import userEvent from '@testing-library/user-event';

describe('<ThemeToggle />', () => {
  // see https://github.com/pacocoursey/next-themes/issues/21#issuecomment-946796898
  let localStorageMock: { [key: string]: string } = {};

  beforeAll(() => {
    // Create a mock of the window.matchMedia function
    global.matchMedia = jest.fn((query) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: jest.fn(),
      removeListener: jest.fn(),
      addEventListener: jest.fn(),
      removeEventListener: jest.fn(),
      dispatchEvent: jest.fn(),
    }));

    // Create mocks of localStorage getItem and setItem functions
    global.Storage.prototype.getItem = jest.fn(
      (key: string) => localStorageMock[key]
    );
    global.Storage.prototype.setItem = jest.fn((key: string, value: string) => {
      localStorageMock[key] = value;
    });
  });

  beforeEach(() => {
    // Clear the localStorage-mock
    localStorageMock = {};
  });

  it('should toggle theme from light to dark', async () => {
    render(
      <ThemeProvider attribute='class' defaultTheme='light'>
        <ThemeToggle />
      </ThemeProvider>
    );

    const button = screen.getByRole('button', { name: /toggle theme/i });
    await userEvent.click(button);

    expect(document.documentElement.style.colorScheme).toEqual('dark');
  });

  it('should toggle theme from dark to light', async () => {
    render(
      <ThemeProvider defaultTheme='dark'>
        <ThemeToggle />
      </ThemeProvider>
    );

    const button = screen.getByRole('button', { name: /toggle theme/i });
    await userEvent.click(button);

    expect(document.documentElement.style.colorScheme).toEqual('light');
  });
});
