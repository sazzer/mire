Cypress.Commands.add("seedData", (data) => {
  cy.wrap(data.sql()).then((sql) => {
    cy.wrap(data.binds()).then((binds) => {
      cy.task("db:seed", { sql, binds });
    });
  });
});
