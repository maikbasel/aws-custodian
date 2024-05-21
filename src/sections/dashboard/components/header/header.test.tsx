import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import Header from '@/sections/dashboard/components/header/header';
import { SWRConfig } from 'swr';
import { getProfiles } from '@/modules/profiles/application/get-profiles';
import { Ok } from 'oxide.ts';
import { Profile } from '@/modules/profiles/core/domain';
import { DIContextProvider } from '@/context/di-context';

jest.mock('@/modules/profiles/application/get-profiles', () => ({
  ...jest.requireActual('@/modules/profiles/application/get-profiles'),
  getProfiles: jest.fn(),
}));

const mockGetProfiles = getProfiles as jest.MockedFunction<typeof getProfiles>;

const profileSet: Profile[] = [
  {
    name: 'prof1',
    credentials: {
      access_key_id: 'key1',
      secret_access_key: 'secret1',
    },
    config: {
      region: 'region1',
      output_format: 'format1',
    },
  },
];

describe('<Header />', () => {
  beforeEach(() => {
    mockGetProfiles.mockResolvedValue(
      Ok({
        profiles: profileSet,
      })
    );
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('should render the Header component without crashing', () => {
    render(
      <DIContextProvider>
        <Header />
      </DIContextProvider>
    );

    waitFor(() =>
      expect(screen.getByTestId('app-header-label')).toBeInTheDocument()
    );
  });

  it('should render the MobileSidebar component within the Header component', () => {
    render(
      <DIContextProvider>
        <Header />
      </DIContextProvider>
    );

    waitFor(() =>
      expect(screen.getByTestId('mobile-app-header-label')).toBeInTheDocument()
    );
  });
});
