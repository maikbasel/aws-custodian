import { render, screen, waitFor } from '@testing-library/react';
import Profiles from './page';
import { ProfileProvider } from '@/sections/dashboard/context/profile-context';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { SWRConfig } from 'swr';
import React from 'react';
import { ProfileSet } from '@/modules/profiles/domain';

describe('Profiles', () => {
  afterEach(() => {
    clearMocks();
  });

  test('should render loading state', () => {
    render(<Profiles />);

    const loadingElement = screen.getByText(/loading/i);
    expect(loadingElement).toBeInTheDocument();
  });

  test('should render error state', async () => {
    const profileSet: ProfileSet = {
      profiles: [],
      errors: {
        err1: ['error1'],
      },
    };
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <Profiles />
        </ProfileProvider>
      </SWRConfig>
    );
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    await expect(screen.findByText('error1')).rejects.toThrowError();
  });
});
