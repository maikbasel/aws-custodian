'use client';

import { ProfileDataSPI } from '@/modules/profiles/core/domain';
import React, { createContext, useMemo } from 'react';
import { createProfileDataAdapter } from '@/modules/profiles/infrastructure/profile-data-adapter';

type DIContextState = {
  profileDataSPI: ProfileDataSPI;
};

export const DIContext = createContext<DIContextState>({} as DIContextState);

export const DIContextProvider = ({ children }: React.PropsWithChildren) => {
  const profileDataSPI = useMemo(() => createProfileDataAdapter(), []);
  return (
    <DIContext.Provider value={{ profileDataSPI }}>
      {children}
    </DIContext.Provider>
  );
};
