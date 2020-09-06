import { User } from "./user";
import { UserModel } from "./api";
import debug from "debug";
import { request } from "../http";

/** The logger to use */
const LOGGER = debug("mire:api:users:load");

/**
 * Update the user details as provided
 * @param user The details of the user to save
 */
export async function saveUser(user: User): Promise<User> {
  LOGGER("Saving user: %o", user);

  const userResponse = await request<UserModel>(`/users/{id}`, {
    urlParams: {
      id: user.id,
    },
    method: "PATCH",
    body: {
      email: user.email,
      display_name: user.displayName,
    },
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
