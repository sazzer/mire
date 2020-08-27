import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";

/** The logger to use */
const LOGGER = debug("mire:api:authentication:start");

/**
 * Start authentication with the named provider
 * @param provider The name of the provider to start authentication with
 */
export function authenticate(provider: string) {
  const template = UrlTemplate.parse(
    env("URL_BASE") + "/authentication/{provider}"
  );
  const url = template.expand({ provider });

  LOGGER("Redirecting to %s", url);

  const eventListener = (event: MessageEvent) => {
    if (event && event.data && event.data.type === "mireAuthenticated") {
      window.removeEventListener("message", eventListener);
      console.log(event.data);
      console.log(event.origin);
    }
  };
  window.addEventListener("message", eventListener);

  window.open(url, "mireAuthentication");
}
