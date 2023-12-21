'use client';

import Link from "next/link";
import useNavigation from "@/hooks/Navigation";
import {Icon} from "@iconify/react";

import React from 'react';

type MenuItemProps = {
    href: string;
    isActive: boolean;
    iconActive: string;
    iconInactive: string;
    label: string;
};

const MenuItem = ({href, isActive, iconActive, iconInactive, label}: MenuItemProps) => {
    return (
        <Link href={href}
              className="flex flex-row space-x-4 items-center px-4 py-3 duration-200 hover:bg-white/10 w-full" >
            {isActive ? (<Icon icon={iconActive} width="38" height="38" />) : <Icon icon={iconInactive} width="38" height="38" />}
            <span className={`text-2xl pt-2 hidden md:flex ${isActive ? 'font-bold' : ''}`}>{label}</span>
        </Link>
    )
}

export default function SidebarMenu() {
    const {
        isHomeActive,
        isProfilesActive,
    } = useNavigation();

    return (
        <nav
            className="flex-col space-y-4 items-center py-8 hidden sm:flex border-r border-zinc-700 h-full  w-[120px] md:w-[250px] md:items-start fixed bg-gray-700">
            <MenuItem href="/" isActive={isHomeActive} iconActive="ant-design:home-filled"
                            iconInactive="ant-design:home-outlined" label="Home"/>
            <MenuItem href="/profiles" isActive={isProfilesActive} iconActive="ant-design:profile-filled"
                            iconInactive="ant-design:profile-outlined" label="Profiles"/>
        </nav>
    );
}