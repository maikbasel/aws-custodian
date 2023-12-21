import React from 'react';
import {render, screen} from '@testing-library/react';
import SidebarMenu from './SidebarMenu';
import Link from "next/link";
import {Icon} from "@iconify/react";
import renderer from 'react-test-renderer';

describe('<SidebarMenu />', () => {
    const mockUseNavigation = jest.fn();

    jest.mock('@/hooks/Navigation', () => ({
        __esModule: true,
        useNavigation: mockUseNavigation(),
    }));

    afterEach(() => {
        jest.clearAllMocks();
    });

    it('should render the Home menu item', () => {
        render(<SidebarMenu/>);

        const homeMenuItem = screen.getByText('Home');
        expect(homeMenuItem).toBeInTheDocument();
    });

    it('should render the Profiles menu item', () => {
        render(<SidebarMenu/>);

        const profilesMenuItem = screen.getByText('Profiles');
        expect(profilesMenuItem).toBeInTheDocument();
    });

    it('should render inactive Home icon when isHomeActive is false', () => {
        mockUseNavigation.mockImplementation(() => ({
            isHomeActive: false,
            isProfilesActive: false,
        }));

        const tree = renderer
            .create(<Link href="/"
                          className="flex flex-row space-x-4 items-center px-4 py-3 duration-200 hover:bg-white/10 w-full">
                <Icon icon="ant-design:home-outlined" width="38" height="38"/>
                <span className="text-2xl pt-2 hidden md:flex">Home</span>
            </Link>)
            .toJSON();

        expect(tree).toMatchSnapshot();
    });

    it('should render active Home icon when isHomeActive is true', () => {
        mockUseNavigation.mockImplementation(() => ({
            isHomeActive: true,
            isProfilesActive: false,
        }));

        const tree = renderer
            .create(<Link href="/"
                          className="flex flex-row space-x-4 items-center px-4 py-3 duration-200 hover:bg-white/10 w-full">
                <Icon icon="ant-design:home-filled" width="38" height="38" />
                <span className="text-2xl pt-2 hidden md:flex">Home</span>
            </Link>)
            .toJSON();

        expect(tree).toMatchSnapshot();
    });

    it('should render inactive Profiles icon when isProfilesActive is false', () => {
        mockUseNavigation.mockImplementation(() => ({
            isHomeActive: false,
            isProfilesActive: false,
        }));

        const tree = renderer
            .create(<Link href="/"
                          className="flex flex-row space-x-4 items-center px-4 py-3 duration-200 hover:bg-white/10 w-full">
                <Icon icon="ant-design:profile-outlined" width="38" height="38"/>
                <span className="text-2xl pt-2 hidden md:flex">Home</span>
            </Link>)
            .toJSON();

        expect(tree).toMatchSnapshot();
    });

    it('should render active Profiles icon when isProfilesActive is true', () => {
        mockUseNavigation.mockImplementation(() => ({
            isHomeActive: false,
            isProfilesActive: true,
        }));

        const tree = renderer
            .create(<Link href="/"
                          className="flex flex-row space-x-4 items-center px-4 py-3 duration-200 hover:bg-white/10 w-full">
                <Icon icon="ant-design:profile-filled" width="38" height="38" />
                <span className="text-2xl pt-2 hidden md:flex">Home</span>
            </Link>)
            .toJSON();

        expect(tree).toMatchSnapshot();
    });

    it('should render Home menu item with correct href', () => {
        render(<SidebarMenu/>);

        const homeMenuItem = screen.getByRole('link', {name: /home/i});
        expect(homeMenuItem).toHaveAttribute('href', '/');
    });

    it('should render Profiles menu item with correct href', () => {
        render(<SidebarMenu/>);

        const profilesMenuItem = screen.getByRole('link', {name: /profiles/i});
        expect(profilesMenuItem).toHaveAttribute('href', '/profiles');
    });
});