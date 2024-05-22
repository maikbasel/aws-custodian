import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { ProfileSet } from '@/modules/profiles/core/domain';
import { ProfileDataTable } from '@/sections/profiles/components/profile-data-table';
import userEvent from '@testing-library/user-event';
import { SWRConfig } from 'swr';
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

describe('<ProfileDataTable />', () => {
  const profileSet: ProfileSet = {
    profiles: [
      {
        name: 'prof1',
        credentials: {
          access_key_id: 'key1',
          secret_access_key: 'secret1',
        },
        config: {
          region: 'eu-west-1',
          output_format: 'json',
        },
      },
    ],
  };

  beforeEach(() => {
    mockValidateCredentials.mockResolvedValue(Ok(false));
  });

  it('should render profile data table with profile set data', async () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={profileSet} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );
    const row = screen.getByText(/prof1/i).closest('tr');

    await waitFor(() => {
      expect(row).toHaveTextContent('key1');
      expect(row).toHaveTextContent('secret1');
      expect(row).toHaveTextContent('eu-west-1');
      expect(row).toHaveTextContent('json');
    });
  });

  it('should select all rows when the "Select All" checkbox is clicked', async () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={profileSet} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));

    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    expect(row1Checkbox).toBeChecked();
  });

  it('given "Select All" checkbox is already checked then should unselect all rows when the "Select All" checkbox is clicked again', async () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={profileSet} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));

    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });
    expect(row1Checkbox).toBeChecked();

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));
    expect(row1Checkbox).not.toBeChecked();
  });

  it('should select row 1 when the "Select row 1" checkbox is clicked', async () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={profileSet} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );
    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    await userEvent.click(row1Checkbox);

    expect(row1Checkbox).toBeChecked();
  });

  it('given row 1 is checkbox is already selected then should unselect row 1 when the "Select row 1" checkbox is clicked again', async () => {
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={profileSet} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );
    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    await userEvent.click(row1Checkbox);
    expect(row1Checkbox).toBeChecked();

    await userEvent.click(row1Checkbox);
    expect(row1Checkbox).not.toBeChecked();
  });

  it('should filter row by region when the "Region" filter is changed', async () => {
    const input: ProfileSet = {
      profiles: [
        {
          name: 'prof1',
          credentials: {
            access_key_id: 'key1',
            secret_access_key: 'secret1',
          },
          config: {
            region: 'eu-west-1',
            output_format: 'json',
          },
        },
        {
          name: 'prof2',
          credentials: {
            access_key_id: 'key2',
            secret_access_key: 'secret2',
          },
          config: {
            region: 'eu-east-1',
            output_format: 'table',
          },
        },
      ],
    };
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={input} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );

    expect(screen.getAllByRole('row').length).toBe(3);

    await userEvent.click(
      screen.getAllByRole('button', { name: /Region/i })[0]
    );

    await userEvent.click(screen.getByRole('option', { name: /eu-west-1/i }));

    expect(screen.getAllByRole('row').length).toBe(2);
  });

  it('should filter row by output format when the "Output Format" filter is changed', async () => {
    const input: ProfileSet = {
      profiles: [
        {
          name: 'prof1',
          credentials: {
            access_key_id: 'key1',
            secret_access_key: 'secret1',
          },
          config: {
            region: 'eu-west-1',
            output_format: 'json',
          },
        },
        {
          name: 'prof2',
          credentials: {
            access_key_id: 'key2',
            secret_access_key: 'secret2',
          },
          config: {
            region: 'eu-east-1',
            output_format: 'table',
          },
        },
      ],
    };
    render(
      <SWRConfig value={{ provider: () => new Map() }}>
        <TooltipProvider>
          <DIContextProvider>
            <ProfileDataTable data={input} />
          </DIContextProvider>
        </TooltipProvider>
      </SWRConfig>
    );

    expect(screen.getAllByRole('row').length).toBe(3);

    await userEvent.click(
      screen.getAllByRole('button', { name: /Output Format/i })[0]
    );

    await userEvent.click(screen.getByRole('option', { name: /json/i }));

    expect(screen.getAllByRole('row').length).toBe(2);
  });
});
