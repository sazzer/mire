import { Breadcrumb } from "./breadcrumb";
import React from "react";
import { Router } from "react-router-dom";
import { axe } from "jest-axe";
import { createMemoryHistory } from "history";
import { render } from "@testing-library/react";

describe("<Breadcrumb>", () => {
  test("No crumbs", async () => {
    const history = createMemoryHistory();
    const { container } = render(
      <Router history={history}>
        <Breadcrumb current="Current" />
      </Router>
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <nav
          aria-label="Current"
        >
          <ol
            class="breadcrumb"
          >
            <li
              aria-current="page"
              class="breadcrumb-item active"
            >
              Current
            </li>
          </ol>
        </nav>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("Empty crumbs", async () => {
    const history = createMemoryHistory();
    const { container } = render(
      <Router history={history}>
        <Breadcrumb current="Current" crumbs={[]} />
      </Router>
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <nav
          aria-label="Current"
        >
          <ol
            class="breadcrumb"
          >
            <li
              aria-current="page"
              class="breadcrumb-item active"
            >
              Current
            </li>
          </ol>
        </nav>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("Some crumbs", async () => {
    const history = createMemoryHistory();
    const { container } = render(
      <Router history={history}>
        <Breadcrumb
          current="Current"
          crumbs={[
            { link: "/", text: "Home" },
            { link: "/abc", text: "Abc" },
          ]}
        />
      </Router>
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <nav
          aria-label="Current"
        >
          <ol
            class="breadcrumb"
          >
            <li
              class="breadcrumb-item"
            >
              <a
                href="/"
              >
                Home
              </a>
            </li>
            <li
              class="breadcrumb-item"
            >
              <a
                href="/abc"
              >
                Abc
              </a>
            </li>
            <li
              aria-current="page"
              class="breadcrumb-item active"
            >
              Current
            </li>
          </ol>
        </nav>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
