import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { CatIcon } from 'lucide-react';
import {
  NavItem,
  SideNav,
} from '@/sections/dashboard/components/sidebar/side-nav';

describe('<SideNav />', function () {
  const items: NavItem[] = [
    {
      isChidren: false,
      title: 'Test Item',
      href: '/test',
      icon: CatIcon,
      color: 'red',
    },
  ];

  it('should render with passed nav items', () => {
    render(<SideNav items={items} setOpen={jest.fn()} className='testClass' />);

    const item = screen.getByText(/Test Item/i);
    expect(item).toBeInTheDocument();
  });

  it('should trigger setOpen on click', () => {
    const mockSetOpen = jest.fn();
    render(
      <SideNav items={items} setOpen={mockSetOpen} className='testClass' />
    );
    const item = screen.getByText(/Test Item/i);

    fireEvent.click(item);

    expect(mockSetOpen).toHaveBeenCalled();
  });
});
