import { useEffect, useState } from 'react';
import { usePathname } from 'next/navigation';

const useNavigation = () => {
  const pathname = usePathname();
  const [isHomeActive, setIsHomeActive] = useState(false);
  const [isProfilesActive, setIsProfilesActive] = useState(false);

  useEffect(() => {
    setIsHomeActive(false);
    setIsProfilesActive(false);

    switch (pathname) {
      case '/':
        setIsHomeActive(true);
        break;
      case '/profiles':
        setIsProfilesActive(true);
        break;
      default:
        break;
    }
  }, [pathname]);

  return {
    isHomeActive,
    isProfilesActive: isProfilesActive,
  };
};

export default useNavigation;
