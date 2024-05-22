import React from 'react';
import { render, screen } from '@testing-library/react';
import TestCredentialsButton from './test-credentials-button';
import { SWRConfig } from 'swr';
import userEvent from '@testing-library/user-event';
import { TooltipProvider } from '@/components/ui/tooltip';
import { validateCredentials } from '@/modules/credentials/application/validateCredentials';
import { Ok } from 'oxide.ts';
import { DIContextProvider } from '@/context/di-context';

jest.mock('@/modules/credentials/application/validateCredentials', () => ({
  ...jest.requireActual(
    '@/modules/credentials/application/validateCredentials'
  ),
  validateCredentials: jest.fn(),
}));

const mockValidateCredentials = validateCredentials as jest.MockedFunction<
  typeof validateCredentials
>;

describe('<TestCredentialsButton />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('should render shield alert icon when profile credentials are invalid', async () => {
    jest.isolateModules(async () => {
      mockValidateCredentials.mockResolvedValue(Ok(false));
      render(
        <SWRConfig value={{ provider: () => new Map() }}>
          <TooltipProvider>
            <DIContextProvider>
              <TestCredentialsButton profile='prof1' />
            </DIContextProvider>
          </TooltipProvider>
        </SWRConfig>
      );

      const button = screen.getByRole('button');
      await userEvent.click(button);

      const shieldSvg = button.querySelector('svg');
      expect(shieldSvg).toHaveClass('lucide-shield-alert');
    });
  });

  it('should render shield check icon when profile credentials are valid', async () => {
    jest.isolateModules(async () => {
      mockValidateCredentials.mockResolvedValue(Ok(true));
      render(
        <SWRConfig value={{ provider: () => new Map() }}>
          <TooltipProvider>
            <DIContextProvider>
              <TestCredentialsButton profile='prof1' />
            </DIContextProvider>
          </TooltipProvider>
        </SWRConfig>
      );

      const button = screen.getByRole('button');
      await userEvent.click(button);

      const shieldSvg = button.querySelector('svg');
      expect(shieldSvg).toHaveClass('lucide-shield-check');
    });
  });
});
