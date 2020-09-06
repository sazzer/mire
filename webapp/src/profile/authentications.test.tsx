import { Authentications } from "./authentications";
import React from "react";
import { axe } from "jest-axe";
import { render } from "@testing-library/react";

describe("<Authentications>", () => {
  test("No authentication details", async () => {
    const user = {
      id: "",
      version: "",
      created: new Date(),
      updated: new Date(),
      displayName: "",
      email: "",
      authentications: [],
    };

    const { container } = render(<Authentications user={user} />);
    expect(container).toMatchInlineSnapshot(`
      <div>
        <ul
          class="list-group"
        />
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("Single Google authentication details", async () => {
    const user = {
      id: "",
      version: "",
      created: new Date(),
      updated: new Date(),
      displayName: "",
      email: "",
      authentications: [
        {
          displayName: "test@example.com",
          authenticationProvider: "google",
          authenticationId: "123",
        },
      ],
    };

    const { container } = render(<Authentications user={user} />);
    expect(container).toMatchInlineSnapshot(`
      <div>
        <ul
          class="list-group"
        >
          <li
            class="list-group-item d-flex justify-content-between align-items-center"
          >
            test@example.com
            <span
              class="btn-google badge"
            >
              <span
                class="fa fa-google"
              />
               
              Google
            </span>
          </li>
        </ul>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
