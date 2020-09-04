import { Before } from "cypress-cucumber-preprocessor/steps";

Before(() => {
  cy.visit("/", {
    onBeforeLoad: (win) => {
      win.sessionStorage.clear();
    },
  });
});
