import React from 'react';
import { render, screen } from '@testing-library/react';
import { ProfileSet } from '@/modules/profiles/domain';
import { ProfileDataTable } from '@/sections/profiles/components/profile-data-table';
import userEvent from '@testing-library/user-event';

describe('<ProfileDataTable />', () => {
  const profileSet: ProfileSet = {
    profiles: {
      prof1: {
        credentials: {
          access_key_id: 'key1',
          secret_access_key: 'secret1',
        },
        config: {
          region: 'eu-west-1',
          output_format: 'json',
        },
      },
    },
    errors: {},
  };

  it('should render profile data table with profile set data', () => {
    render(<ProfileDataTable data={profileSet} />);

    const row = screen.getByText(/prof1/i).closest('tr');

    expect(row).toHaveTextContent('key1');
    expect(row).toHaveTextContent('secret1');
    expect(row).toHaveTextContent('eu-west-1');
    expect(row).toHaveTextContent('json');
  });

  it('should select all rows when the "Select All" checkbox is clicked', async () => {
    render(<ProfileDataTable data={profileSet} />);

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));

    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    expect(row1Checkbox).toBeChecked();
  });

  it('given "Select All" checkbox is already checked then should unselect all rows when the "Select All" checkbox is clicked again', async () => {
    render(<ProfileDataTable data={profileSet} />);

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));

    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });
    expect(row1Checkbox).toBeChecked();

    await userEvent.click(screen.getByRole('checkbox', { name: 'Select all' }));
    expect(row1Checkbox).not.toBeChecked();
  });

  it('should select row 1 when the "Select row 1" checkbox is clicked', async () => {
    render(<ProfileDataTable data={profileSet} />);
    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    await userEvent.click(row1Checkbox);

    expect(row1Checkbox).toBeChecked();
  });

  it('given row 1 is checkbox is already selected then should unselect row 1 when the "Select row 1" checkbox is clicked again', async () => {
    render(<ProfileDataTable data={profileSet} />);
    const row1Checkbox = screen.getByRole('checkbox', { name: 'Select row 1' });

    await userEvent.click(row1Checkbox);
    expect(row1Checkbox).toBeChecked();

    await userEvent.click(row1Checkbox);
    expect(row1Checkbox).not.toBeChecked();
  });
});
