import React, { useContext, useState } from "react";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("mire:currentUser");

type User = string;

/**
 * The shape of the actual context store
 */
export interface UserContext {
  /** The user details */
  user: User | null;
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/** The actual context type */
const userContext = React.createContext<UserContext>({
  user: null,
  setUserId: () => {},
  clearUserId: () => {},
});

export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);

  const contextValue = {
    user,
    setUserId: (userId: string) => {
      LOGGER("Loading user: %s", userId);
      setUser(userId);
    },
    clearUserId: () => {
      setUser(null);
    },
  };

  return (
    <userContext.Provider value={contextValue}>{children}</userContext.Provider>
  );
};

/**
 * Hook to access the user details
 */
export function useUser() {
  const context = useContext(userContext);

  return {
    user: context.user,
    hasUser: context.user !== null,
    setUserId: context.setUserId,
    clearUserId: context.clearUserId,
  };
}
