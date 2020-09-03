import React from "react";
import { Spinner } from "./spinner";
import { axe } from "jest-axe";
import { render } from "@testing-library/react";

test("<Spinner>", async () => {
  const { container } = render(<Spinner />);
  expect(container).toMatchInlineSnapshot(`
    <div>
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
  `);
  expect(await axe(container)).toHaveNoViolations();
});
