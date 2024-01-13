import { render, fireEvent, screen } from '@testing-library/react';
import Sidebar from '@/sections/dashboard/components/sidebar/sidebar';
import React from 'react';

describe('<Sidebar />', () => {
  afterEach(() => {
    jest.restoreAllMocks();
  });

  it('should render Sidebar without throwing an error', () => {
    const sidebar = render(<Sidebar />);
    expect(sidebar).toBeTruthy();
  });

  it('given sidebar is closed when the toggle button is clicked then should toggle to open ', () => {
    const mockHandleToggle = jest.fn();
    jest.spyOn(React, 'useState').mockReturnValue([false, mockHandleToggle]);

    render(<Sidebar />);

    const button = screen.getByRole('button');
    fireEvent.click(button);

    expect(mockHandleToggle).toHaveBeenCalledWith(true);
  });

  it('should apply transition to nav when the toggle button is clicked', () => {
    render(<Sidebar />);
    const button = screen.getByRole('button');

    fireEvent.click(button);

    const nav = screen.getByTestId('sidebar-nav');
    expect(nav.className).toContain('duration-500');
  });
});
