function getButton(provider, callback) {
  const button = cy.findByText(`Sign In with ${provider}`);
  return button;
}

Cypress.Commands.add("getAuthentication", (callback) => {
  cy.findByLabelText("Login / Register").within(() => {
    callback({
      getButton,
    });
  });
});
