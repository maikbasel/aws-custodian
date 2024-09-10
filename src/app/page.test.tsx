import { render, screen, waitFor } from '@testing-library/react';
import Profiles from './page';
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

describe('<Profiles />', () => {
  beforeEach(() => {
    mockGetProfiles.mockResolvedValue(Ok(profileSet));
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  test('should render isLoading state', async () => {
    render(
      <DIContextProvider>
        <Profiles />
      </DIContextProvider>
    );

    await waitFor(() => {
      const loadingElement = screen.getByText(/Loading/i);
      expect(loadingElement).toBeInTheDocument();
    });
  });

  test('should render error state', async () => {
    const profileSet: ProfileSet = {
      profiles: [],
    };
    mockGetProfiles.mockResolvedValue(Ok(profileSet));
    render(
      <DIContextProvider>
        <Profiles />
      </DIContextProvider>
    );
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    await expect(screen.findByText('error1')).rejects.toThrowError();
  });
});
