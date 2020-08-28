import { User, UserId } from "./user";

/**
 * Load the user details from the server with the given ID
 * @param id The ID of the user to load
 */
export async function loadUser(id: UserId): Promise<User> {
  return {
    id,
    version: "version",
    created: new Date(),
    updated: new Date(),
    displayName: "Test User",
    email: "testuser@example.com",
    authentications: [],
  };
}
