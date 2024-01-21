import React from 'react';
import { render, screen } from '@testing-library/react';
import { ProfileSet } from '@/modules/profiles/domain';
import { ProfileDataTable } from '@/sections/profiles/components/profile-data-table';

describe('<ProfileDataTable />', () => {
  it('should render profile data table with profile set data', () => {
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

    render(<ProfileDataTable data={profileSet} />);

    const row = screen.getByText(/prof1/i).closest('tr');

    expect(row).toHaveTextContent('key1');
    expect(row).toHaveTextContent('secret1');
    expect(row).toHaveTextContent('eu-west-1');
    expect(row).toHaveTextContent('json');
  });
});
