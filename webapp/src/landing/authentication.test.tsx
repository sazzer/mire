import { render, waitFor } from "@testing-library/react";

import { Authentication } from "./authentication";
import React from "react";
import { axe } from "jest-axe";
import { listAuthenticationProviders } from "../api/authentication";
import { resourceCache } from "use-async-resource";

jest.mock("../api/authentication");

describe("<Authentication>", () => {
  const listAuthenticationProvidersMock = listAuthenticationProviders as jest.Mock;

  beforeEach(() => {
    resourceCache(listAuthenticationProviders).clear();
    listAuthenticationProvidersMock.mockClear();
  });

  test("Before authentication providers are loaded", async () => {
    listAuthenticationProvidersMock.mockReturnValueOnce(new Promise(() => {}));

    const { container } = render(<Authentication />);
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          aria-labelledby="authenticationLabel"
        >
          <h2
            id="authenticationLabel"
          >
            Login / Register
          </h2>
          <div
            class="text-center"
          >
            <div
              class="spinner-border"
              role="status"
              style="width: 3rem; height: 3rem;"
            >
              <span
                class="sr-only"
              >
                Loading...
              </span>
            </div>
          </div>
        </div>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("After some authentication providers are loaded", async () => {
    listAuthenticationProvidersMock.mockResolvedValueOnce([
      "facebook",
      "twitter",
    ]);

    const { container } = render(<Authentication />);
    await waitFor(
      () => expect(listAuthenticationProvidersMock).toHaveBeenCalledTimes(1),
      {
        container,
      }
    );

    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          aria-labelledby="authenticationLabel"
        >
          <h2
            id="authenticationLabel"
          >
            Login / Register
          </h2>
          <button
            class="btn btn-block btn-social btn-facebook"
          >
            <span
              class="fa fa-facebook"
            />
             
            Sign In with Facebook
          </button>
          <button
            class="btn btn-block btn-social btn-twitter"
          >
            <span
              class="fa fa-twitter"
            />
             
            Sign In with Twitter
          </button>
        </div>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("After no authentication providers are loaded", async () => {
    listAuthenticationProvidersMock.mockResolvedValueOnce([]);

    const { container } = render(<Authentication />);
    await waitFor(() =>
      expect(listAuthenticationProvidersMock).toHaveBeenCalledTimes(1)
    );

    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          aria-labelledby="authenticationLabel"
        >
          <h2
            id="authenticationLabel"
          >
            Login / Register
          </h2>
        </div>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
