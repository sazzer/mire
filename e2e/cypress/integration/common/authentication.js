import { Then, When } from "cypress-cucumber-preprocessor/steps";

Then("the available authentication providers are:", (dataTable) => {
  cy.getAuthentication(({ getButton }) => {
    dataTable.rawTable
      .map((row) => row[0])
      .forEach((provider) => {
        getButton(provider).should("be.visible");
      });
  });
});

When("I authenticate using {string}", (provider) => {
  cy.getAuthentication(({ getButton }) => {
    getButton(provider).should("be.visible").click();
  });
});

Then("I am logged in as {string}", (displayName) => {
  cy.getPageHeader(({ getUserMenu }) => {
    getUserMenu(({ getDropdownButton }) => {
      getDropdownButton().should("be.visible").should("have.text", displayName);
    });
  });
});
