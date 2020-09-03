import { User, UserId } from "./user";

import { UserModel } from "./api";
import debug from "debug";
import { request } from "../http";

/** The logger to use */
const LOGGER = debug("mire:api:users:load");

/**
 * Load the user details from the server with the given ID
 * @param id The ID of the user to load
 */
export async function loadUser(id: UserId): Promise<User> {
  LOGGER("Loading user: %s", id);

  const userResponse = await request<UserModel>(`/users/${id}`);
  const userModel = userResponse.body!!;

  return {
    id: userModel.id,
    version: userResponse.headers.get("Etag")!!,
    created: userModel.created,
    updated: userModel.updated,
    displayName: userModel.displayName,
    email: userModel.email,
    authentications: userModel.authentications,
  };
}
