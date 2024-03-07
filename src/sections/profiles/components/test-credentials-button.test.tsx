import React from 'react';
import { render, screen } from '@testing-library/react';
import TestCredentialsButton from './test-credentials-button';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { SWRConfig } from 'swr';
import userEvent from '@testing-library/user-event';

describe('<TestCredentialsButton />', () => {
  afterEach(() => {
    clearMocks();
  });

  it('should render shield alert icon when profile credentials are invalid', async () => {
    mockIPC((cmd) => {
      if (cmd === 'validate_credentials') {
        return false;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TestCredentialsButton profile='prof1' />
      </SWRConfig>
    );

    const button = screen.getByRole('button');
    await userEvent.click(button);

    const shieldSvg = button.querySelector('svg');
    expect(shieldSvg).toHaveClass('lucide-shield-alert');
  });

  it('should render shield check icon when profile credentials are valid', async () => {
    mockIPC((cmd) => {
      if (cmd === 'validate_credentials') {
        return true;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TestCredentialsButton profile='prof1' />
      </SWRConfig>
    );

    const button = screen.getByRole('button');
    await userEvent.click(button);

    const shieldSvg = button.querySelector('svg');
    expect(shieldSvg).toHaveClass('lucide-shield-check');
  });
});
