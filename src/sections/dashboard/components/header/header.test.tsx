import React from 'react';
import { render, screen } from '@testing-library/react';
import Header from '@/sections/dashboard/components/header/header';

describe('<Header />', () => {
  it('should render the Header component without crashing', () => {
    render(<Header />);
    expect(screen.getByTestId('app-header-label')).toBeInTheDocument();
  });

  it('should render the MobileSidebar component within the Header component', () => {
    render(<Header />);
    expect(screen.getByTestId('mobile-app-header-label')).toBeInTheDocument();
  });
});
