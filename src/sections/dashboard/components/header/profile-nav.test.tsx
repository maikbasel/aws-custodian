import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { randomFillSync } from 'crypto';
import {
  ProfileNav,
  ProfileSet,
} from '@/sections/dashboard/components/header/profile-nav';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { ProfileProvider } from '@/sections/dashboard/context/profile-context';
import { SWRConfig } from 'swr';

describe('<ProfileNav />', () => {
  const profileSet: ProfileSet = {
    profiles: {
      prof1: {
        credentials: {
          access_key_id: 'key1',
          secret_access_key: 'secret1',
        },
        config: {
          region: 'region1',
          output_format: 'format1',
        },
      },
      prof2: {
        credentials: {
          access_key_id: 'key2',
          secret_access_key: 'secret2',
        },
        config: {
          region: 'region2',
          output_format: 'format2',
        },
      },
    },
    errors: {
      err1: ['error1'],
    },
  };

  // jsdom doesn't come with a WebCrypto implementation
  beforeAll(() => {
    Object.defineProperty(window, 'crypto', {
      value: {
        // eslint-disable-next-line
        // @ts-ignore
        getRandomValues: (buffer) => {
          return randomFillSync(buffer);
        },
      },
    });
  });

  afterEach(() => {
    clearMocks();
  });

  it('should render the component without error when in loading state', async () => {
    render(<ProfileNav />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('should expand profile nav when dropdown is clicked', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const triggerButton = screen.getByTestId('profile-nav-trigger');

    expect(triggerButton.getAttribute('aria-expanded')).toBe('false');
    await userEvent.click(triggerButton);

    expect(
      screen.getByTestId('profile-nav-trigger').getAttribute('aria-expanded')
    ).toBe('true');
  });

  it('should render chevron down icon in trigger button when profile nav is closed and additional profiles are available', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const triggerButton = screen.getByTestId('profile-nav-trigger');

    const chevronSvg = triggerButton.querySelector('svg');
    expect(chevronSvg).toHaveClass('lucide-chevron-down');
  });

  it('should render chevron up icon in trigger button when profile nav is open and additional profiles are available', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const triggerButton = screen.getByTestId('profile-nav-trigger');
    await userEvent.click(triggerButton);

    const chevronSvg = triggerButton.querySelector('svg');
    expect(chevronSvg).toHaveClass('lucide-chevron-up');
  });

  it('should not render chevron icon when no additional profiles are available', async () => {
    const singleProfile: ProfileSet = {
      profiles: {
        prof1: {
          credentials: {
            access_key_id: 'key1',
            secret_access_key: 'secret1',
          },
          config: {
            region: 'region1',
            output_format: 'format1',
          },
        },
      },
      errors: {},
    };
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return singleProfile;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const triggerButton = screen.getByTestId('profile-nav-trigger');
    const chevronSvg = triggerButton.querySelector('svg .lucide-chevron-up');

    expect(chevronSvg).not.toBeInTheDocument();
  });

  it('should render placeholder values for region and format label in profile nav trigger when these optional values are not set', async () => {
    const inputProfileSet: ProfileSet = {
      profiles: {
        prof1: {
          credentials: {
            access_key_id: 'key1',
            secret_access_key: 'secret1',
          },
          config: {
            region: undefined,
            output_format: undefined,
          },
        },
      },
      errors: {},
    };
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return inputProfileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const profileNavTriggerRegionLabel = screen.getByTestId(
      'profile-nav-trigger-region-label'
    );
    expect(profileNavTriggerRegionLabel).toHaveTextContent('?');
    const profileNavTriggerFormatLabel = screen.getByTestId(
      'profile-nav-trigger-format-label'
    );
    expect(profileNavTriggerFormatLabel).toHaveTextContent('?');
  });

  it('should render placeholder values for region and format label in profile nav item when these optional values are not set', async () => {
    const inputProfileSet: ProfileSet = {
      profiles: {
        prof1: {
          credentials: {
            access_key_id: 'key1',
            secret_access_key: 'secret1',
          },
          config: {
            region: 'region1',
            output_format: 'format1',
          },
        },
        prof2: {
          credentials: {
            access_key_id: 'key2',
            secret_access_key: 'secret2',
          },
          config: {
            region: undefined,
            output_format: undefined,
          },
        },
      },
      errors: {},
    };
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return inputProfileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    const triggerButton = screen.getByTestId('profile-nav-trigger');
    await userEvent.click(triggerButton);

    const profileNavTriggerRegionLabel = screen.getByTestId(
      'prof2-profile-nav-item-region-label'
    );
    expect(profileNavTriggerRegionLabel).toHaveTextContent('?');
    const profileNavTriggerFormatLabel = screen.getByTestId(
      'prof2-profile-nav-item-format-label'
    );
    expect(profileNavTriggerFormatLabel).toHaveTextContent('?');
  });

  it('should select clicked profile as current when profile nav item is clicked', async () => {
    mockIPC((cmd) => {
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <ProfileProvider>
          <ProfileNav />
        </ProfileProvider>
      </SWRConfig>
    );
    // await waitForElementToBeRemoved(() => screen.queryByText('Loading...')); see https://github.com/testing-library/react-testing-library/issues/865
    await waitFor(() => {
      expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    await userEvent.click(screen.getByTestId('profile-nav-trigger'));

    const profileNavItem = screen.getByText(/prof2/i);
    await userEvent.click(profileNavItem);

    const triggerButton = screen.getByRole('button', {
      name: /prof2/i,
    });
    expect(triggerButton).toBeInTheDocument();
    expect(triggerButton.getAttribute('data-testid')).toEqual(
      'profile-nav-trigger'
    );
  });
});
