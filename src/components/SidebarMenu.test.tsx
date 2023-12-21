import React from 'react';
import { render, screen } from '@testing-library/react';
import SidebarMenu from './SidebarMenu';
import Link from 'next/link';
import { Icon } from '@iconify/react';
import renderer from 'react-test-renderer';
import useNavigation from "@/hooks/Navigation";

jest.mock('@/hooks/Navigation');
const mockUseNavigation = useNavigation as jest.MockedFunction<typeof useNavigation>

describe('<SidebarMenu />', () => {


  afterEach(() => {
    jest.clearAllMocks();
  });

  it('should render the Home menu item', () => {
    mockUseNavigation.mockReturnValue({
      isHomeActive: false,
      isProfilesActive: false,
    });

    render(<SidebarMenu />);

    const homeMenuItem = screen.getByText('Home');
    expect(homeMenuItem).toBeInTheDocument();
  });

  it('should render the Profiles menu item', () => {
    mockUseNavigation.mockReturnValue({
      isHomeActive: false,
      isProfilesActive: false,
    });

    render(<SidebarMenu />);

    const profilesMenuItem = screen.getByText('Profiles');
    expect(profilesMenuItem).toBeInTheDocument();
  });

  it('should render inactive Home icon when isHomeActive is false', () => {
    mockUseNavigation.mockReturnValue({
      isHomeActive: false,
      isProfilesActive: false,
    });

    const tree = renderer
      .create(
        <Link
          href='/'
          className='flex w-full flex-row items-center space-x-4 px-4 py-3 duration-200 hover:bg-white/10'
        >
          <Icon icon='ant-design:home-outlined' width='38' height='38' />
          <span className='hidden pt-2 text-2xl md:flex'>Home</span>
        </Link>
      )
      .toJSON();

    expect(tree).toMatchSnapshot();
  });

  it('should render active Home icon when isHomeActive is true', () => {
    mockUseNavigation.mockReturnValue({
      isHomeActive: true,
      isProfilesActive: false,
    });

    const tree = renderer
      .create(
        <Link
          href='/'
          className='flex w-full flex-row items-center space-x-4 px-4 py-3 duration-200 hover:bg-white/10'
        >
          <Icon icon='ant-design:home-filled' width='38' height='38' />
          <span className='hidden pt-2 text-2xl md:flex font-bold'>Home</span>
        </Link>
      )
      .toJSON();

    expect(tree).toMatchSnapshot();
  });

  it('should render inactive Profiles icon when isProfilesActive is false', () => {
    mockUseNavigation.mockImplementation(() => ({
      isHomeActive: false,
      isProfilesActive: false,
    }));

    const tree = renderer
      .create(
        <Link
          href='/'
          className='flex w-full flex-row items-center space-x-4 px-4 py-3 duration-200 hover:bg-white/10'
        >
          <Icon icon='ant-design:profile-outlined' width='38' height='38' />
          <span className='hidden pt-2 text-2xl md:flex'>Home</span>
        </Link>
      )
      .toJSON();

    expect(tree).toMatchSnapshot();
  });

  it('should render active Profiles icon when isProfilesActive is true', () => {
    mockUseNavigation.mockReturnValue({
      isHomeActive: false,
      isProfilesActive: true,
    });

    const tree = renderer
      .create(
        <Link
          href='/'
          className='flex w-full flex-row items-center space-x-4 px-4 py-3 duration-200 hover:bg-white/10'
        >
          <Icon icon='ant-design:profile-filled' width='38' height='38' />
          <span className='hidden pt-2 text-2xl md:flex font-bold'>Home</span>
        </Link>
      )
      .toJSON();

    expect(tree).toMatchSnapshot();
  });

  it('should render Home menu item with correct href', () => {
    render(<SidebarMenu />);

    const homeMenuItem = screen.getByRole('link', { name: /home/i });
    expect(homeMenuItem).toHaveAttribute('href', '/');
  });

  it('should render Profiles menu item with correct href', () => {
    render(<SidebarMenu />);

    const profilesMenuItem = screen.getByRole('link', { name: /profiles/i });
    expect(profilesMenuItem).toHaveAttribute('href', '/profiles');
  });
});
