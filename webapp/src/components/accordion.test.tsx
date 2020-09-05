import { fireEvent, render } from "@testing-library/react";

import { Accordion } from "./accordion";
import React from "react";
import { axe } from "jest-axe";

describe("<Accordion>", () => {
  test("No panes", async () => {
    const { container } = render(<Accordion panes={[]} id="test" />);
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="accordion"
          id="test"
        />
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("Some panes", async () => {
    const { container } = render(
      <Accordion
        panes={[
          {
            title: "First title",
            content: <div>First pane</div>,
          },
          {
            title: "Second title",
            content: <div>Second pane</div>,
          },
        ]}
        id="test"
      />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="accordion"
          id="test"
        >
          <div
            class="card"
          >
            <div
              class="card-header"
              id="test-card-0-heading"
            >
              <h2
                class="mb-0"
              >
                <button
                  aria-controls="test-card-0"
                  aria-expanded="true"
                  class="btn btn-link btn-block text-left "
                  type="button"
                >
                  First title
                </button>
              </h2>
            </div>
            <div
              aria-labelledby="test-card-0-heading"
              class="collapse show"
              id="test-card-0"
            >
              <div
                class="card-body"
              >
                <div>
                  First pane
                </div>
              </div>
            </div>
          </div>
          <div
            class="card"
          >
            <div
              class="card-header"
              id="test-card-1-heading"
            >
              <h2
                class="mb-0"
              >
                <button
                  aria-controls="test-card-1"
                  aria-expanded="false"
                  class="btn btn-link btn-block text-left collapsed"
                  type="button"
                >
                  Second title
                </button>
              </h2>
            </div>
            <div
              aria-labelledby="test-card-1-heading"
              class="collapse"
              id="test-card-1"
            >
              <div
                class="card-body"
              />
            </div>
          </div>
        </div>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });

  test("Open second pane", async () => {
    const { container, getByText } = render(
      <Accordion
        panes={[
          {
            title: "First title",
            content: <div>First pane</div>,
          },
          {
            title: "Second title",
            content: <div>Second pane</div>,
          },
        ]}
        id="test"
      />
    );
    fireEvent(
      getByText("Second title"),
      new MouseEvent("click", {
        bubbles: true,
        cancelable: true,
      })
    );

    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="accordion"
          id="test"
        >
          <div
            class="card"
          >
            <div
              class="card-header"
              id="test-card-0-heading"
            >
              <h2
                class="mb-0"
              >
                <button
                  aria-controls="test-card-0"
                  aria-expanded="false"
                  class="btn btn-link btn-block text-left collapsed"
                  type="button"
                >
                  First title
                </button>
              </h2>
            </div>
            <div
              aria-labelledby="test-card-0-heading"
              class="collapse"
              id="test-card-0"
            >
              <div
                class="card-body"
              />
            </div>
          </div>
          <div
            class="card"
          >
            <div
              class="card-header"
              id="test-card-1-heading"
            >
              <h2
                class="mb-0"
              >
                <button
                  aria-controls="test-card-1"
                  aria-expanded="true"
                  class="btn btn-link btn-block text-left "
                  type="button"
                >
                  Second title
                </button>
              </h2>
            </div>
            <div
              aria-labelledby="test-card-1-heading"
              class="collapse show"
              id="test-card-1"
            >
              <div
                class="card-body"
              >
                <div>
                  Second pane
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    `);
    expect(await axe(container)).toHaveNoViolations();
  });
});
