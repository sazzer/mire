import { Then } from "cypress-cucumber-preprocessor/steps";

Then("the available authentication providers are:", (dataTable) => {
  cy.getAuthentication(({ getButton }) => {
    dataTable.rawTable
      .map((row) => row[0])
      .forEach((provider) => {
        getButton(provider).should("be.visible");
      });
  });
});
