import React from 'react';
import {
  render,
  screen,
  waitForElementToBeRemoved,
} from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { randomFillSync } from 'crypto';
import {
  ProfileNav,
  ProfileSet,
} from '@/sections/dashboard/components/header/profile-nav';
import { mockIPC } from '@tauri-apps/api/mocks';
import { ProfileProvider } from '@/sections/dashboard/context/profile-context';

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

  it('should render the component without error when in loading state', async () => {
    render(<ProfileNav />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('Should trigger state change when dropdown is clicked', async () => {
    mockIPC((cmd) => {
      // simulated rust command called "get_profiles" that returns profile records
      if (cmd === 'get_profiles') {
        return profileSet;
      }
    });
    render(
      <ProfileProvider>
        <ProfileNav />
      </ProfileProvider>
    );
    await waitForElementToBeRemoved(() => screen.queryByText('Loading...'));

    const triggerButton = screen.getByTestId('profile-nav-trigger');

    // initial state closed
    expect(triggerButton.getAttribute('aria-expanded')).toBe('false');
    // open dropdown
    await userEvent.click(triggerButton);

    expect(
      screen.getByTestId('profile-nav-trigger').getAttribute('aria-expanded')
    ).toBe('true');
  });
});
