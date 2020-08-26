import { render, waitFor } from "@testing-library/react";

import { Authentication } from "./authentication";
import React from "react";
import { listAuthenticationProviders } from "../api/authentication";

jest.mock("../api/authentication");

describe("<Authentication>", () => {
  const listAuthenticationProvidersMock = listAuthenticationProviders as jest.Mock;

  beforeEach(() => {
    listAuthenticationProvidersMock.mockClear();
  });

  test("Before authentication providers are loaded", () => {
    listAuthenticationProvidersMock.mockReturnValueOnce(new Promise(() => {}));

    const { container } = render(<Authentication />);
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div>
          <h2>
            Login / Register
          </h2>
        </div>
      </div>
    `);
  });

  test("After some authentication providers are loaded", async () => {
    listAuthenticationProvidersMock.mockReturnValueOnce(
      Promise.resolve(["facebook", "twitter"])
    );

    const { container } = render(<Authentication />);
    await waitFor(() =>
      expect(listAuthenticationProvidersMock).toHaveBeenCalledTimes(1)
    );

    expect(container).toMatchInlineSnapshot(`
      <div>
        <div>
          <h2>
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
  });

  test("After no authentication providers are loaded", async () => {
    listAuthenticationProvidersMock.mockReturnValueOnce(Promise.resolve([]));

    const { container } = render(<Authentication />);
    await waitFor(() =>
      expect(listAuthenticationProvidersMock).toHaveBeenCalledTimes(1)
    );

    expect(container).toMatchInlineSnapshot(`
      <div>
        <div>
          <h2>
            Login / Register
          </h2>
        </div>
      </div>
    `);
  });
});
