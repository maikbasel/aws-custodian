import React from 'react';
import { render, screen } from '@testing-library/react';
import BreadCrumb from '@/components/breadcrumb';

describe('<Breadcrumb />', () => {
  test('renders dashboard link', () => {
    render(<BreadCrumb items={[]} />);

    expect(screen.getByText('Dashboard')).toBeInTheDocument();
  });

  test('renders correct number of items', () => {
    const items = [
      { title: 'Item 1', link: '/item-1' },
      { title: 'Item 2', link: '/item-2' },
    ];
    render(<BreadCrumb items={items} />);

    expect(screen.getAllByRole('link').length).toBe(3); // dashboard + 2 items
  });

  test('last item is not clickable', () => {
    const items = [
      { title: 'Item 1', link: '/item-1' },
      { title: 'Item 2', link: '/item-2' },
    ];
    render(<BreadCrumb items={items} />);

    expect(screen.getAllByRole('link')[2]).toHaveClass('pointer-events-none');
  });

  test('renders separators between items', () => {
    const items = [
      { title: 'Item 1', link: '/item-1' },
      { title: 'Item 2', link: '/item-2' },
    ];

    render(<BreadCrumb items={items} />);

    const chevrons = screen.getAllByRole('separator');
    expect(chevrons.length).toBe(items.length);
  });
});
