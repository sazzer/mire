function getUserMenu(callback) {
  cy.findByRole("menu", { selector: "li" }).within(($menu) => {
    if (callback) {
      callback({
        getElement: () => $menu,
        getDropdownButton: () => cy.get('a[data-toggle="dropdown"]'),
      });
    }
  });
}

Cypress.Commands.add("getPageHeader", (callback) => {
  cy.findByRole("banner", { selector: "header" }).within(() => {
    callback({
      getUserMenu,
    });
  });
});
