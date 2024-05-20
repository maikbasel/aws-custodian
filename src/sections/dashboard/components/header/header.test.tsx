import React from 'react';
import { render, screen } from '@testing-library/react';
import Header from '@/sections/dashboard/components/header/header';
import { mockIPC } from '@tauri-apps/api/mocks';
import { SWRConfig } from 'swr';

const profiles = [
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
  it('should render the Header component without crashing', () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return {
          profiles,
        };
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <Header />
      </SWRConfig>
    );

    expect(screen.getByTestId('app-header-label')).toBeInTheDocument();
  });

  it('should render the MobileSidebar component within the Header component', () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return {
          profiles,
        };
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <Header />
      </SWRConfig>
    );

    expect(screen.getByTestId('mobile-app-header-label')).toBeInTheDocument();
  });
});
