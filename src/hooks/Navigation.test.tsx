import useNavigation from "./Navigation"
import {renderHook} from '@testing-library/react';
import {usePathname} from "next/navigation";

jest.mock('next/navigation');

describe('useNavigation hook', () => {
    let usePathnameMock: jest.Mock;

    beforeEach(() => {
        usePathnameMock = usePathname as jest.Mock;
        usePathnameMock.mockReset();
    });

    test('should correctly set isHomeActive when pathname is /', () => {
        usePathnameMock.mockReturnValue('/');

        const { result } = renderHook(() => useNavigation());

        expect(result.current.isHomeActive).toBe(true);
    });
    
    test('should correctly set isProfilesActive when pathname is /profiles', () => {
        usePathnameMock.mockReturnValue('/profiles');

        const { result } = renderHook(() => useNavigation());

        expect(result.current.isProfilesActive).toBe(true);
    });

    test('should correctly reset isHomeActive and isProfilesActive on pathname change', () => {
        usePathnameMock.mockReturnValue('/');
        const { result, rerender } = renderHook(() => useNavigation());
        expect(result.current.isHomeActive).toBe(true);

        usePathnameMock.mockReturnValue('/newPath');
        rerender();

        expect(result.current.isHomeActive).toBe(false);
        expect(result.current.isProfilesActive).toBe(false);
    });
});