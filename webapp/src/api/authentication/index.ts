import debug from "debug";
import { request } from "../http";

/** The logger to use */
const LOGGER = debug("mire:api:authentication");

/** The type representing the list of providers */
export type Providers = Array<string>;

/**
 * Get the list of authentication providers that we are able to use.
 */
export async function listAuthenticationProviders(): Promise<Providers> {
  const providers = await request<Providers>("/authentication");
  LOGGER("Available authentication providers: %o", providers);
  return providers.body ?? [];
}
