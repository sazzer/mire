import React, { useContext, useEffect, useState } from "react";
import { User, UserId, loadUser } from "./api/users";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("mire:currentUser");

/** The key into Session Storage where the current user ID is stored */
const USER_KEY = "mire_current_user";

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
  const loadUserDetails = async (userId: string | null) => {
    if (userId) {
      try {
        const loadedUser = await loadUser(userId);
        LOGGER("Loaded user: %o", loadedUser);
        setUser(loadedUser);
      } catch (e) {
        LOGGER("Failed to load user %s: %o", userId, e);
        setUser(null);
      }
    } else {
      setUser(null);
    }
  };

  useEffect(() => {
    const storedUserId = sessionStorage.getItem(USER_KEY);
    if (storedUserId) {
      LOGGER("Loading remembered user: %s", storedUserId);
      loadUserDetails(storedUserId);
    }
  }, []);

  const contextValue = {
    user,
    setUserId: (userId: string) => {
      LOGGER("Loading user: %s", userId);
      sessionStorage.setItem(USER_KEY, userId);
      loadUserDetails(userId);
    },
    clearUserId: () => {
      sessionStorage.removeItem(USER_KEY);
      loadUserDetails(null);
    },
  };

  return (
    <userContext.Provider value={contextValue}>{children}</userContext.Provider>
  );
};

export interface UserHookBase {
  setUserId: (userId: UserId) => void;
  clearUserId: () => void;
}
export interface UserHookWithUser extends UserHookBase {
  hasUser: true;
  user: User;
  userId: UserId;
}

export interface UserHookWithoutUser extends UserHookBase {
  hasUser: false;
}

export type UserHook = UserHookWithUser | UserHookWithoutUser;
/**
 * Hook to access the user details
 */
export function useUser(): UserHook {
  const context = useContext(userContext);

  const user = context.user;
  if (user) {
    return {
      hasUser: true,
      user,
      userId: user.id,
      setUserId: context.setUserId,
      clearUserId: context.clearUserId,
    };
  } else {
    return {
      hasUser: false,
      setUserId: context.setUserId,
      clearUserId: context.clearUserId,
    };
  }
}
