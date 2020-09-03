/**
 * Representation of a single authentication with a provider on the API
 */
export interface AuthenticationModel {
  /** The provider that this authentication is with */
  authenticationProvider: string;
  /** The ID of the user with the provider */
  authenticationId: string;
  /** The display name of this authentication */
  displayName: string;
}

/**
 * Representation of a single user on the API
 */
export interface UserModel {
  /** The ID of the user */
  id: string;
  /** When the user was created */
  created: Date;
  /** When the user was last updated */
  updated: Date;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user */
  email: string;
  /** The set of authentication details for the user */
  authentications: AuthenticationModel[];
}
