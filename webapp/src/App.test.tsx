import { App } from "./App";
import React from "react";
import { Router } from "react-router-dom";
import { createMemoryHistory } from "history";
import { render } from "@testing-library/react";

test("renders learn react link", () => {
  const history = createMemoryHistory();

  const { getByText } = render(
    <Router history={history}>
      <App />
    </Router>
  );
  const linkElement = getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});
