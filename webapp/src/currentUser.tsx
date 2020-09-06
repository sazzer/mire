import React, { useContext, useEffect, useState } from "react";
import { User, UserId, loadUser } from "./api/users";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("mire:currentUser");

/** The key into Session Storage where the current user ID is stored */
const USER_KEY = "mire_current_user";

/**
 * Base type for the structure of the context store
 */
export interface UserContextBase {
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/**
 * Shape of the context store when a user is present
 */
export interface UserContextWithUser extends UserContextBase {
  user: User;
  reload: () => Promise<User>;
}

/**
 * Shape of the context store when a user is not present
 */
export interface UserContextWithoutUser extends UserContextBase {
  user: null;
}

/**
 * The shape of the actual context store
 */
export type UserContext = UserContextWithUser | UserContextWithoutUser;

/** The actual context type */
const userContext = React.createContext<UserContext>({
  user: null,
  setUserId: () => {},
  clearUserId: () => {},
});

/**
 * React component to act as a context provider for the current user.
 */
export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);

  const loadUserDetails = async (userId: string, force: boolean) => {
    const loadedUser = await loadUser(userId, force);
    sessionStorage.setItem(USER_KEY, userId);

    if (loadedUser.version !== user?.version) {
      LOGGER("Loaded user: %o", loadedUser);
      setUser(loadedUser);
      return loadedUser;
    }
    return user;
  };

  const clearUserDetails = () => {
    LOGGER("Cleared user");
    sessionStorage.removeItem(USER_KEY);
    setUser(null);
  };

  useEffect(
    () => {
      const storedUserId = sessionStorage.getItem(USER_KEY);
      if (storedUserId) {
        LOGGER("Loading remembered user: %s", storedUserId);
        loadUserDetails(storedUserId, false);
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  let contextValue: UserContext = {
    user: null,
    setUserId: (userId) => loadUserDetails(userId, false),
    clearUserId: clearUserDetails,
  };

  if (user) {
    contextValue = {
      user,
      setUserId: (userId) => loadUserDetails(userId, false),
      clearUserId: clearUserDetails,
      reload: () => loadUserDetails(user.id, true),
    };
  }

  return (
    <userContext.Provider value={contextValue}>{children}</userContext.Provider>
  );
};

/**
 * Base type for the shape of the return value from the useUser hook.
 */
export interface UserHookBase {
  setUserId: (userId: UserId) => void;
  clearUserId: () => void;
}

/**
 * Shape of the return value from the useUser hook when a user is present
 */
export interface UserHookWithUser extends UserHookBase {
  hasUser: true;
  user: User;
  userId: UserId;
  reload: () => Promise<User>;
}

/**
 * Shape of the return value from the useUser hook when a user is not present
 */
export interface UserHookWithoutUser extends UserHookBase {
  hasUser: false;
}

/**
 * Shape of the return value from the useUser hook
 */
export type UserHook = UserHookWithUser | UserHookWithoutUser;

/**
 * Hook to access the user details
 */
export function useUser(): UserHook {
  const context = useContext(userContext);

  if (context.user) {
    return {
      hasUser: true,
      user: context.user,
      userId: context.user.id,
      setUserId: context.setUserId,
      clearUserId: context.clearUserId,
      reload: context.reload,
    };
  } else {
    return {
      hasUser: false,
      setUserId: context.setUserId,
      clearUserId: context.clearUserId,
    };
  }
}
