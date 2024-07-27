'use client';

import { ProfileDataSPI } from '@/modules/profiles/core/domain';
import React, { createContext, useMemo } from 'react';
import { createProfileDataAdapter } from '@/modules/profiles/infrastructure/profile-data-adapter';
import { createCredentialsDataAdapter } from '@/modules/credentials/infrastructure/credentials-data-adapter';
import { CredentialsDataSPI } from '@/modules/credentials/core/domain';
import { createParameterDataAdapter } from '@/modules/parameters/infrastructure/parameter-data-adapter';
import { ParameterDataSPI } from '@/modules/parameters/core/domain';

type DIContextState = {
  profileDataSPI: ProfileDataSPI;
  credentialsDataSPI: CredentialsDataSPI;
  parameterDataSPI: ParameterDataSPI;
};

export const DIContext = createContext<DIContextState>({} as DIContextState);

export const DIContextProvider = ({ children }: React.PropsWithChildren) => {
  const profileDataSPI = useMemo(() => createProfileDataAdapter(), []);
  const credentialsDataSPI = useMemo(() => createCredentialsDataAdapter(), []);
  const parameterDataSPI = useMemo(() => createParameterDataAdapter(), []);

  return (
    <DIContext.Provider
      value={{ profileDataSPI, credentialsDataSPI, parameterDataSPI }}
    >
      {children}
    </DIContext.Provider>
  );
};
