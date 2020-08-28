/** The type representing the ID of a user */
export type UserId = string;

/**
 * Representation of a single authentication with a provider
 */
export interface Authentication {
  /** The provider that this authentication is with */
  provider: string;
  /** The ID of the user with the provider */
  id: string;
  /** The display name of this authentication */
  displayName: string;
}

/**
 * Representation of a single user
 */
export interface User {
  /** The ID of the user */
  id: UserId;
  /** The version of the user record */
  version: string;
  /** When the user was created */
  created: Date;
  /** When the user was last updated */
  updated: Date;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user */
  email: string;
  /** The set of authentication details for the user */
  authentications: Authentication[];
}
