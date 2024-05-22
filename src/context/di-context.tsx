'use client';

import { ProfileDataSPI } from '@/modules/profiles/core/domain';
import React, { createContext, useMemo } from 'react';
import { createProfileDataAdapter } from '@/modules/profiles/infrastructure/profile-data-adapter';
import { createCredentialsDataAdapter } from '@/modules/credentials/infrastructure/credentials-data-adapter';
import { CredentialsDataSPI } from '@/modules/credentials/core/domain';

type DIContextState = {
  profileDataSPI: ProfileDataSPI;
  credentialsDataSPI: CredentialsDataSPI;
};

export const DIContext = createContext<DIContextState>({} as DIContextState);

export const DIContextProvider = ({ children }: React.PropsWithChildren) => {
  const profileDataSPI = useMemo(() => createProfileDataAdapter(), []);
  const credentialsDataSPI = useMemo(() => createCredentialsDataAdapter(), []);

  return (
    <DIContext.Provider value={{ profileDataSPI, credentialsDataSPI }}>
      {children}
    </DIContext.Provider>
  );
};
