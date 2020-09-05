import { User, UserId } from "./user";

import { UserModel } from "./api";
import debug from "debug";
import { request } from "../http";

/** The logger to use */
const LOGGER = debug("mire:api:users:load");

/**
 * Load the user details from the server with the given ID
 * @param id The ID of the user to load
 * @param force Whether to force loading the user
 */
export async function loadUser(id: UserId, force?: boolean): Promise<User> {
  LOGGER("Loading user: %s", id);

  const userResponse = await request<UserModel>(`/users/{id}`, {
    urlParams: {
      id,
    },
    ignoreCache: force || false,
  });
  const userModel = userResponse.body!!;

  return {
    id: userModel.id,
    version: userResponse.headers.get("Etag")!!,
    created: new Date(userModel.created),
    updated: new Date(userModel.updated),
    displayName: userModel.displayName,
    email: userModel.email,
    authentications: userModel.authentications,
  };
}
