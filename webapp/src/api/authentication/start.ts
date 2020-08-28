import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";
import { storeToken } from "../http/token";

/** The logger to use */
const LOGGER = debug("mire:api:authentication:start");

/**
 * Start authentication with the named provider
 * @param provider The name of the provider to start authentication with
 * @param setUserId Callback to provide the User ID of the newly authenticated user
 */
export function authenticate(
  provider: string,
  setUserId: (userId: string) => void
) {
  const template = UrlTemplate.parse(
    env("URL_BASE") + "/authentication/{provider}"
  );
  const url = template.expand({ provider });

  LOGGER("Redirecting to %s", url);

  const eventListener = (event: MessageEvent) => {
    if (event && event.data && event.data.type === "mireAuthenticated") {
      window.removeEventListener("message", eventListener);
      if (event.data.accessToken && event.data.expires) {
        storeToken(event.data.accessToken, event.data.expires);
      }
      if (event.data.user) {
        setUserId(event.data.user);
      }
    }
  };
  window.addEventListener("message", eventListener);

  window.open(url, "mireAuthentication");
}
