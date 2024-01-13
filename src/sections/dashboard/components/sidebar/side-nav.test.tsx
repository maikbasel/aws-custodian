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
      hasChildren: false,
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

  it('should trigger setOpen on nav item click', () => {
    const mockSetOpen = jest.fn();
    render(
      <SideNav items={items} setOpen={mockSetOpen} className='testClass' />
    );
    const item = screen.getByText(/Test Item/i);

    fireEvent.click(item);

    expect(mockSetOpen).toHaveBeenCalled();
  });

  it('should render parent item with child items when hasChildren is true', () => {
    const nestedItems: NavItem[] = [
      {
        title: 'Test Parent Item',
        href: '/parent',
        icon: CatIcon,
        color: 'blue',
        hasChildren: true,
        children: [
          {
            hasChildren: false,
            title: 'Test Child Item',
            href: '/child',
            icon: CatIcon,
            color: 'red',
          },
        ],
      },
    ];
    render(
      <SideNav items={nestedItems} setOpen={jest.fn()} className='testClass' />
    );

    const parentItem = screen.getByText(/Test Parent Item/i);
    expect(parentItem).toBeInTheDocument();

    fireEvent.click(parentItem);

    const childItem = screen.getByText(/Test Child Item/i);
    expect(childItem).toBeInTheDocument();
  });

  it('should trigger setOpen on nested nav item click', () => {
    const mockSetOpen = jest.fn();
    const nestedItems: NavItem[] = [
      {
        title: 'Test Parent Item',
        href: '/parent',
        icon: CatIcon,
        color: 'blue',
        hasChildren: true,
        children: [
          {
            hasChildren: false,
            title: 'Test Child Item',
            href: '/child',
            icon: CatIcon,
            color: 'red',
          },
        ],
      },
    ];
    render(
      <SideNav
        items={nestedItems}
        setOpen={mockSetOpen}
        className='testClass'
      />
    );

    const parentItem = screen.getByText(/Test Parent Item/i);
    fireEvent.click(parentItem);
    const childItem = screen.getByText(/Test Child Item/i);
    fireEvent.click(childItem);

    expect(mockSetOpen).toHaveBeenCalledTimes(1);
  });
});
