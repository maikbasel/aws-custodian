import { render, screen, waitFor } from '@testing-library/react';
import Profiles from './page';
import { mockIPC } from '@tauri-apps/api/mocks';
import { SWRConfig } from 'swr';
import React from 'react';
import { ProfileSet } from '@/modules/profiles/core/domain';
import { getProfiles } from '@/modules/profiles/application/get-profiles';
import { Ok } from 'oxide.ts';
import { DIContextProvider } from '@/context/di-context';

jest.mock('@/modules/profiles/application/get-profiles', () => ({
  ...jest.requireActual('@/modules/profiles/application/get-profiles'),
  getProfiles: jest.fn(),
}));

const mockGetProfiles = getProfiles as jest.MockedFunction<typeof getProfiles>;

const profileSet: ProfileSet = {
  profiles: [],
};

describe('Profiles', () => {
  beforeEach(() => {
    mockGetProfiles.mockResolvedValue(Ok(profileSet));
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  test('should render loading state', () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <DIContextProvider>
          <Profiles />
        </DIContextProvider>
      </SWRConfig>
    );

    const loadingElement = screen.getByText(/loading/i);
    expect(loadingElement).toBeInTheDocument();
  });

  test('should render error state', async () => {
    const profileSet: ProfileSet = {
      profiles: [],
    };
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <DIContextProvider>
          <Profiles />
        </DIContextProvider>
      </SWRConfig>
    );
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    await expect(screen.findByText('error1')).rejects.toThrowError();
  });
});
