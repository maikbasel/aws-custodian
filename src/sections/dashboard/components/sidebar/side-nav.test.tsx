import React from 'react';
import { fireEvent, render, screen } from '@testing-library/react';
import { CatIcon } from 'lucide-react';
import {
  NavItem,
  SideNav,
} from '@/sections/dashboard/components/sidebar/side-nav';

const mockUsePathname = jest.fn();

jest.mock('next/navigation', () => ({
  usePathname() {
    return mockUsePathname();
  },
}));

describe('<SideNav />', function () {
  const items: NavItem[] = [
    {
      hasChildren: false,
      title: 'Test Item',
      href: '/test',
      icon: CatIcon,
      color: 'red',
    },
  ];

  afterEach(() => {
    jest.clearAllMocks();
  });

  it('should render with passed nav items', () => {
    render(<SideNav items={items} setOpen={jest.fn()} className='testClass' />);

    const item = screen.getByText(/Test Item/i);
    expect(item).toBeInTheDocument();
  });

  it('should trigger setOpen on nav item click', () => {
    const mockSetOpen = jest.fn();
    render(
      <SideNav items={items} setOpen={mockSetOpen} className='testClass' />
    );
    const item = screen.getByText(/Test Item/i);

    fireEvent.click(item);

    expect(mockSetOpen).toHaveBeenCalled();
  });

  it('should display muted title text when current path equals nav item href', () => {
    mockUsePathname.mockReturnValue('/test');
    const mockSetOpen = jest.fn();
    render(
      <SideNav items={items} setOpen={mockSetOpen} className='testClass' />
    );
    const anchor = screen.getByRole('link', { name: /Test Item/i });

    expect(anchor).toHaveClass('bg-muted', 'font-bold', 'hover:bg-muted');
  });
});
